//! GGA_X_LV_RPW86 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lv_rpw86.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_lv_rpw86_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = t30 * t30;
            let t33 = t28 / t31;
            let t34 = v_rho0 * v_rho0;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / t34;
            let t40 = t33 * v_sigma0 * t38;
            let t42 = f64x8::splat(1.0) + f64x8::splat(0.003931018518518519) * t40;
            let t43 = v_sigma0 * v_sigma0;
            let t44 = t43 * v_sigma0;
            let t45 = t34 * t34;
            let t46 = t45 * t45;
            let t47 = f64x8::splat(1.0) / t46;
            let t48 = t44 * t47;
            let t49 = f64x8::splat(9.704561350131286e-08) * t48;
            let t50 = f64x8::splat(1.0) + t49;
            let t51 = f64x8::splat(1.0) / t50;
            let t54 = t28 * t28;
            let t57 = t54 / t30 / t29;
            let t58 = t45 * v_rho0;
            let t60 = f64x8::splat(1.0) / t35 / t58;
            let t65 = f64x8::splat(1.0) + f64x8::splat(0.077125) * t40 + f64x8::splat(0.030086805555555554) * t57 * t43 * t60 + f64x8::splat(7.26282598747199e-07) * t48;
            let t66 = (simd::pow(t65, f64x8::splat(1.0) / f64x8::splat(15.0)));
            let t67 = f64x8::splat(1.15) + t49;
            let t68 = f64x8::splat(1.0) / t67;
            let t69 = t66 * t68;
            let t72 = t42 * t51 + f64x8::splat(9.704561350131286e-08) * t48 * t69;
            let t76 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t72));
            let t77 = (v_rho1).simd_le(dens_threshold);
            let t78 = -t16;
            let t80 = ((t14).select(t11, (t10).select(t15, t78 * t7)));
            let t81 = f64x8::splat(1.0) + t80;
            let t82 = (t81).simd_le(zeta_threshold);
            let t83 = (simd::cbrt(t81));
            let t85 = ((t82).select(t22, t83 * t81));
            let t86 = t85 * t26;
            let t87 = v_rho1 * v_rho1;
            let t88 = (simd::cbrt(v_rho1));
            let t89 = t88 * t88;
            let t91 = f64x8::splat(1.0) / t89 / t87;
            let t93 = t33 * v_sigma2 * t91;
            let t95 = f64x8::splat(1.0) + f64x8::splat(0.003931018518518519) * t93;
            let t96 = v_sigma2 * v_sigma2;
            let t97 = t96 * v_sigma2;
            let t98 = t87 * t87;
            let t99 = t98 * t98;
            let t100 = f64x8::splat(1.0) / t99;
            let t101 = t97 * t100;
            let t102 = f64x8::splat(9.704561350131286e-08) * t101;
            let t103 = f64x8::splat(1.0) + t102;
            let t104 = f64x8::splat(1.0) / t103;
            let t107 = t98 * v_rho1;
            let t109 = f64x8::splat(1.0) / t88 / t107;
            let t114 = f64x8::splat(1.0) + f64x8::splat(0.077125) * t93 + f64x8::splat(0.030086805555555554) * t57 * t96 * t109 + f64x8::splat(7.26282598747199e-07) * t101;
            let t115 = (simd::pow(t114, f64x8::splat(1.0) / f64x8::splat(15.0)));
            let t116 = f64x8::splat(1.15) + t102;
            let t117 = f64x8::splat(1.0) / t116;
            let t118 = t115 * t117;
            let t121 = t95 * t104 + f64x8::splat(9.704561350131286e-08) * t101 * t118;
            let t125 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t86 * t121));
            let tzk0 = t76 + t125;
            acc_zk = tzk0;
            let t126 = t6 * t6;
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t16 * t127;
            let t130 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t128)));
            let t133 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t130));
            let t134 = t133 * t26;
            let t138 = t26 * t26;
            let t139 = f64x8::splat(1.0) / t138;
            let t140 = t25 * t139;
            let t143 = t5 * t140 * t72 / f64x8::splat(8.0);
            let t144 = t34 * v_rho0;
            let t146 = f64x8::splat(1.0) / t36 / t144;
            let t147 = v_sigma0 * t146;
            let t151 = t50 * t50;
            let t152 = f64x8::splat(1.0) / t151;
            let t153 = t42 * t152;
            let t154 = t46 * v_rho0;
            let t155 = f64x8::splat(1.0) / t154;
            let t156 = t44 * t155;
            let t161 = t66 * t66;
            let t162 = t161 * t161;
            let t164 = t162 * t162;
            let t165 = t164 * t162 * t161;
            let t166 = f64x8::splat(1.0) / t165;
            let t167 = t166 * t68;
            let t170 = t45 * t34;
            let t172 = f64x8::splat(1.0) / t35 / t170;
            let t177 = -f64x8::splat(0.20566666666666666) * t33 * t147 - f64x8::splat(0.16046296296296297) * t57 * t43 * t172 - f64x8::splat(5.810260789977592e-06) * t156;
            let t178 = t167 * t177;
            let t181 = t43 * t43;
            let t182 = t181 * t43;
            let t183 = t46 * t46;
            let t185 = f64x8::splat(1.0) / t183 / v_rho0;
            let t186 = t182 * t185;
            let t187 = t67 * t67;
            let t188 = f64x8::splat(1.0) / t187;
            let t189 = t66 * t188;
            let t192 = -f64x8::splat(0.010482716049382716) * t33 * t147 * t51 + f64x8::splat(7.763649080105029e-07) * t153 * t156 - f64x8::splat(7.763649080105029e-07) * t156 * t69 + f64x8::splat(6.4697075667541905e-09) * t48 * t178 + f64x8::splat(7.534280879876956e-14) * t186 * t189;
            let t197 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t134 * t72 - t143 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t192));
            let t198 = t78 * t127;
            let t200 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t198)));
            let t203 = ((t82).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t83 * t200));
            let t204 = t203 * t26;
            let t208 = t85 * t139;
            let t211 = t5 * t208 * t121 / f64x8::splat(8.0);
            let t213 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t204 * t121 - t211));
            let tvrho0 = t76 + t125 + t6 * (t197 + t213);
            acc_vrho_0 = tvrho0;
            let t217 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t128)));
            let t220 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t217));
            let t221 = t220 * t26;
            let t226 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t221 * t72 - t143));
            let t228 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t198)));
            let t231 = ((t82).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t83 * t228));
            let t232 = t231 * t26;
            let t236 = t87 * v_rho1;
            let t238 = f64x8::splat(1.0) / t89 / t236;
            let t239 = v_sigma2 * t238;
            let t243 = t103 * t103;
            let t244 = f64x8::splat(1.0) / t243;
            let t245 = t95 * t244;
            let t246 = t99 * v_rho1;
            let t247 = f64x8::splat(1.0) / t246;
            let t248 = t97 * t247;
            let t253 = t115 * t115;
            let t254 = t253 * t253;
            let t256 = t254 * t254;
            let t257 = t256 * t254 * t253;
            let t258 = f64x8::splat(1.0) / t257;
            let t259 = t258 * t117;
            let t262 = t98 * t87;
            let t264 = f64x8::splat(1.0) / t88 / t262;
            let t269 = -f64x8::splat(0.20566666666666666) * t33 * t239 - f64x8::splat(0.16046296296296297) * t57 * t96 * t264 - f64x8::splat(5.810260789977592e-06) * t248;
            let t270 = t259 * t269;
            let t273 = t96 * t96;
            let t274 = t273 * t96;
            let t275 = t99 * t99;
            let t277 = f64x8::splat(1.0) / t275 / v_rho1;
            let t278 = t274 * t277;
            let t279 = t116 * t116;
            let t280 = f64x8::splat(1.0) / t279;
            let t281 = t115 * t280;
            let t284 = -f64x8::splat(0.010482716049382716) * t33 * t239 * t104 + f64x8::splat(7.763649080105029e-07) * t245 * t248 - f64x8::splat(7.763649080105029e-07) * t248 * t118 + f64x8::splat(6.4697075667541905e-09) * t101 * t270 + f64x8::splat(7.534280879876956e-14) * t278 * t281;
            let t289 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t232 * t121 - t211 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t86 * t284));
            let tvrho1 = t76 + t125 + t6 * (t226 + t289);
            acc_vrho_1 = tvrho1;
            let t295 = t43 * t47;
            let t306 = f64x8::splat(0.077125) * t33 * t38 + f64x8::splat(0.06017361111111111) * t57 * v_sigma0 * t60 + f64x8::splat(2.178847796241597e-06) * t295;
            let t307 = t167 * t306;
            let t310 = t181 * v_sigma0;
            let t311 = f64x8::splat(1.0) / t183;
            let t312 = t310 * t311;
            let t315 = f64x8::splat(0.003931018518518519) * t33 * t38 * t51 - f64x8::splat(2.9113684050393857e-07) * t153 * t295 + f64x8::splat(2.9113684050393857e-07) * t295 * t69 + f64x8::splat(6.4697075667541905e-09) * t48 * t307 - f64x8::splat(2.825355329953859e-14) * t312 * t189;
            let t319 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t315));
            let tvsigma0 = t6 * t319;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t323 = t96 * t100;
            let t334 = f64x8::splat(0.077125) * t33 * t91 + f64x8::splat(0.06017361111111111) * t57 * v_sigma2 * t109 + f64x8::splat(2.178847796241597e-06) * t323;
            let t335 = t259 * t334;
            let t338 = t273 * v_sigma2;
            let t339 = f64x8::splat(1.0) / t275;
            let t340 = t338 * t339;
            let t343 = f64x8::splat(0.003931018518518519) * t33 * t91 * t104 - f64x8::splat(2.9113684050393857e-07) * t245 * t323 + f64x8::splat(2.9113684050393857e-07) * t323 * t118 + f64x8::splat(6.4697075667541905e-09) * t101 * t335 - f64x8::splat(2.825355329953859e-14) * t340 * t281;
            let t347 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t86 * t343));
            let tvsigma2 = t6 * t347;
            acc_vsigma_2 = tvsigma2;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
