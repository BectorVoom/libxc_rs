//! GGA_C_BMK exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_bmk.c`
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
pub fn gga_c_bmk_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_c_ss_1: f64,
    param_c_ss_2: f64,
    param_c_ss_3: f64,
    param_c_ss_4: f64,
    param_c_ss_0: f64,
    param_c_ab_1: f64,
    param_c_ab_2: f64,
    param_c_ab_3: f64,
    param_c_ab_4: f64,
    param_c_ab_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c_ss_1 = f64x8::splat(param_c_ss_1);
    let param_c_ss_2 = f64x8::splat(param_c_ss_2);
    let param_c_ss_3 = f64x8::splat(param_c_ss_3);
    let param_c_ss_4 = f64x8::splat(param_c_ss_4);
    let param_c_ss_0 = f64x8::splat(param_c_ss_0);
    let param_c_ab_1 = f64x8::splat(param_c_ab_1);
    let param_c_ab_2 = f64x8::splat(param_c_ab_2);
    let param_c_ab_3 = f64x8::splat(param_c_ab_3);
    let param_c_ab_4 = f64x8::splat(param_c_ab_4);
    let param_c_ab_0 = f64x8::splat(param_c_ab_0);
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
        {
            let t2 = v_rho0 - v_rho1;
            let t3 = v_rho0 + v_rho1;
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = t2 * t4;
            let t6 = f64x8::splat(1.0) + t5;
            let t7 = (t6).simd_le(zeta_threshold);
            let t8 = ((v_rho0).simd_le(dens_threshold)) | (t7);
            let t9 = ((t7).select(zeta_threshold, t6));
            let t10 = f64x8::splat(M_CBRT3);
            let t11 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t12 = (simd::cbrt(t11));
            let t13 = t10 * t12;
            let t14 = f64x8::splat(M_CBRT4);
            let t15 = t14 * t14;
            let t16 = t13 * t15;
            let t17 = (simd::cbrt(t3));
            let t18 = f64x8::splat(1.0) / t17;
            let t19 = f64x8::splat(M_CBRT2);
            let t20 = t18 * t19;
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = f64x8::splat(1.0) / t21;
            let t23 = (simd::cbrt(t6));
            let t25 = ((t7).select(t22, f64x8::splat(1.0) / t23));
            let t27 = t16 * t20 * t25;
            let t29 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t27;
            let t30 = ((t27).sqrt());
            let t33 = ((t27) * (t27).sqrt());
            let t35 = t10 * t10;
            let t36 = t12 * t12;
            let t37 = t35 * t36;
            let t38 = t37 * t14;
            let t39 = t17 * t17;
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t19 * t19;
            let t42 = t40 * t41;
            let t43 = t25 * t25;
            let t45 = t38 * t42 * t43;
            let t47 = f64x8::splat(3.79785) * t30 + f64x8::splat(0.8969) * t27 + f64x8::splat(0.204775) * t33 + f64x8::splat(0.123235) * t45;
            let t50 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t47;
            let t51 = (simd::ln(t50));
            let t53 = f64x8::splat(0.062182) * t29 * t51;
            let t55 = t21 * zeta_threshold;
            let t57 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t55, f64x8::splat(2.0) * t19));
            let t59 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t55, f64x8::splat(0.0)));
            let t63 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t19 - f64x8::splat(2.0));
            let t64 = (t57 + t59 - f64x8::splat(2.0)) * t63;
            let t66 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t27;
            let t71 = f64x8::splat(7.05945) * t30 + f64x8::splat(1.549425) * t27 + f64x8::splat(0.420775) * t33 + f64x8::splat(0.1562925) * t45;
            let t74 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t71;
            let t75 = (simd::ln(t74));
            let t79 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t27;
            let t84 = f64x8::splat(5.1785) * t30 + f64x8::splat(0.905775) * t27 + f64x8::splat(0.1100325) * t33 + f64x8::splat(0.1241775) * t45;
            let t87 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t84;
            let t88 = (simd::ln(t87));
            let t89 = t79 * t88;
            let t95 = -t53 + t64 * (-f64x8::splat(0.03109) * t66 * t75 + t53 - f64x8::splat(0.019751789702565206) * t89) + f64x8::splat(0.019751789702565206) * t64 * t89;
            let t98 = ((t8).select(f64x8::splat(0.0), t9 * t95 / f64x8::splat(2.0)));
            let t99 = param_c_ss_0;
            let t100 = param_c_ss_1;
            let t101 = t100 * v_sigma0;
            let t102 = v_rho0 * v_rho0;
            let t103 = (simd::cbrt(v_rho0));
            let t104 = t103 * t103;
            let t106 = f64x8::splat(1.0) / t104 / t102;
            let t107 = v_sigma0 * t106;
            let t109 = f64x8::splat(1.0) + f64x8::splat(0.2) * t107;
            let t110 = f64x8::splat(1.0) / t109;
            let t114 = param_c_ss_2;
            let t115 = v_sigma0 * v_sigma0;
            let t116 = t114 * t115;
            let t117 = t102 * t102;
            let t118 = t117 * v_rho0;
            let t120 = f64x8::splat(1.0) / t103 / t118;
            let t121 = t109 * t109;
            let t122 = f64x8::splat(1.0) / t121;
            let t123 = t120 * t122;
            let t126 = param_c_ss_3;
            let t127 = t115 * v_sigma0;
            let t128 = t126 * t127;
            let t129 = t117 * t117;
            let t130 = f64x8::splat(1.0) / t129;
            let t131 = t121 * t109;
            let t132 = f64x8::splat(1.0) / t131;
            let t133 = t130 * t132;
            let t136 = param_c_ss_4;
            let t137 = t115 * t115;
            let t138 = t136 * t137;
            let t139 = t129 * t102;
            let t141 = f64x8::splat(1.0) / t104 / t139;
            let t142 = t121 * t121;
            let t143 = f64x8::splat(1.0) / t142;
            let t144 = t141 * t143;
            let t147 = t99 + f64x8::splat(0.2) * t101 * t106 * t110 + f64x8::splat(0.04) * t116 * t123 + f64x8::splat(0.008) * t128 * t133 + f64x8::splat(0.0016) * t138 * t144;
            let t148 = t98 * t147;
            let t150 = f64x8::splat(1.0) - t5;
            let t151 = (t150).simd_le(zeta_threshold);
            let t152 = ((v_rho1).simd_le(dens_threshold)) | (t151);
            let t153 = ((t151).select(zeta_threshold, t150));
            let t154 = (simd::cbrt(t150));
            let t156 = ((t151).select(t22, f64x8::splat(1.0) / t154));
            let t158 = t16 * t20 * t156;
            let t160 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t158;
            let t161 = ((t158).sqrt());
            let t164 = ((t158) * (t158).sqrt());
            let t166 = t156 * t156;
            let t168 = t38 * t42 * t166;
            let t170 = f64x8::splat(3.79785) * t161 + f64x8::splat(0.8969) * t158 + f64x8::splat(0.204775) * t164 + f64x8::splat(0.123235) * t168;
            let t173 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t170;
            let t174 = (simd::ln(t173));
            let t176 = f64x8::splat(0.062182) * t160 * t174;
            let t178 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t158;
            let t183 = f64x8::splat(7.05945) * t161 + f64x8::splat(1.549425) * t158 + f64x8::splat(0.420775) * t164 + f64x8::splat(0.1562925) * t168;
            let t186 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t183;
            let t187 = (simd::ln(t186));
            let t191 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t158;
            let t196 = f64x8::splat(5.1785) * t161 + f64x8::splat(0.905775) * t158 + f64x8::splat(0.1100325) * t164 + f64x8::splat(0.1241775) * t168;
            let t199 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t196;
            let t200 = (simd::ln(t199));
            let t201 = t191 * t200;
            let t207 = -t176 + t64 * (-f64x8::splat(0.03109) * t178 * t187 + t176 - f64x8::splat(0.019751789702565206) * t201) + f64x8::splat(0.019751789702565206) * t64 * t201;
            let t210 = ((t152).select(f64x8::splat(0.0), t153 * t207 / f64x8::splat(2.0)));
            let t211 = t100 * v_sigma2;
            let t212 = v_rho1 * v_rho1;
            let t213 = (simd::cbrt(v_rho1));
            let t214 = t213 * t213;
            let t216 = f64x8::splat(1.0) / t214 / t212;
            let t217 = v_sigma2 * t216;
            let t219 = f64x8::splat(1.0) + f64x8::splat(0.2) * t217;
            let t220 = f64x8::splat(1.0) / t219;
            let t224 = v_sigma2 * v_sigma2;
            let t225 = t114 * t224;
            let t226 = t212 * t212;
            let t227 = t226 * v_rho1;
            let t229 = f64x8::splat(1.0) / t213 / t227;
            let t230 = t219 * t219;
            let t231 = f64x8::splat(1.0) / t230;
            let t232 = t229 * t231;
            let t235 = t224 * v_sigma2;
            let t236 = t126 * t235;
            let t237 = t226 * t226;
            let t238 = f64x8::splat(1.0) / t237;
            let t239 = t230 * t219;
            let t240 = f64x8::splat(1.0) / t239;
            let t241 = t238 * t240;
            let t244 = t224 * t224;
            let t245 = t136 * t244;
            let t246 = t237 * t212;
            let t248 = f64x8::splat(1.0) / t214 / t246;
            let t249 = t230 * t230;
            let t250 = f64x8::splat(1.0) / t249;
            let t251 = t248 * t250;
            let t254 = t99 + f64x8::splat(0.2) * t211 * t216 * t220 + f64x8::splat(0.04) * t225 * t232 + f64x8::splat(0.008) * t236 * t241 + f64x8::splat(0.0016) * t245 * t251;
            let t255 = t210 * t254;
            let t257 = t13 * t15 * t18;
            let t259 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t257;
            let t260 = ((t257).sqrt());
            let t263 = ((t257) * (t257).sqrt());
            let t266 = t37 * t14 * t40;
            let t268 = f64x8::splat(3.79785) * t260 + f64x8::splat(0.8969) * t257 + f64x8::splat(0.204775) * t263 + f64x8::splat(0.123235) * t266;
            let t271 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t268;
            let t272 = (simd::ln(t271));
            let t274 = f64x8::splat(0.062182) * t259 * t272;
            let t275 = t2 * t2;
            let t276 = t275 * t275;
            let t277 = t3 * t3;
            let t278 = t277 * t277;
            let t279 = f64x8::splat(1.0) / t278;
            let t280 = t276 * t279;
            let t281 = t23 * t6;
            let t282 = ((t7).select(t55, t281));
            let t283 = t154 * t150;
            let t284 = ((t151).select(t55, t283));
            let t285 = t282 + t284 - f64x8::splat(2.0);
            let t286 = t285 * t63;
            let t288 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t257;
            let t293 = f64x8::splat(7.05945) * t260 + f64x8::splat(1.549425) * t257 + f64x8::splat(0.420775) * t263 + f64x8::splat(0.1562925) * t266;
            let t296 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t293;
            let t297 = (simd::ln(t296));
            let t301 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t257;
            let t306 = f64x8::splat(5.1785) * t260 + f64x8::splat(0.905775) * t257 + f64x8::splat(0.1100325) * t263 + f64x8::splat(0.1241775) * t266;
            let t309 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t306;
            let t310 = (simd::ln(t309));
            let t311 = t301 * t310;
            let t313 = -f64x8::splat(0.03109) * t288 * t297 + t274 - f64x8::splat(0.019751789702565206) * t311;
            let t314 = t286 * t313;
            let t318 = -t274 + t280 * t314 + f64x8::splat(0.019751789702565206) * t286 * t311 - t98 - t210;
            let t320 = param_c_ab_1;
            let t321 = t107 + t217;
            let t322 = t320 * t321;
            let t325 = f64x8::splat(1.0) + f64x8::splat(0.003) * t107 + f64x8::splat(0.003) * t217;
            let t326 = f64x8::splat(1.0) / t325;
            let t329 = param_c_ab_2;
            let t330 = t321 * t321;
            let t331 = t329 * t330;
            let t332 = t325 * t325;
            let t333 = f64x8::splat(1.0) / t332;
            let t336 = param_c_ab_3;
            let t337 = t330 * t321;
            let t338 = t336 * t337;
            let t339 = t332 * t325;
            let t340 = f64x8::splat(1.0) / t339;
            let t343 = param_c_ab_4;
            let t344 = t330 * t330;
            let t345 = t343 * t344;
            let t346 = t332 * t332;
            let t347 = f64x8::splat(1.0) / t346;
            let t350 = param_c_ab_0 + f64x8::splat(0.003) * t322 * t326 + f64x8::splat(9e-06) * t331 * t333 + f64x8::splat(2.7e-08) * t338 * t340 + f64x8::splat(8.1e-11) * t345 * t347;
            let t351 = t318 * t350;
            let tzk0 = t148 + t255 + t351;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
