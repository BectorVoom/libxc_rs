//! MGGA_C_CC vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_cc.c`
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
pub fn mgga_c_cc_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
        {
            let t3 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t4 = v_rho0 + v_rho1;
            let t5 = t4 * t4;
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(t4));
            let t8 = t7 * t7;
            let t10 = f64x8::splat(1.0) / t8 / t6;
            let t11 = t3 * t10;
            let t12 = (simd::cbrt(v_rho0));
            let t13 = t12 * t12;
            let t15 = f64x8::splat(1.0) / t13 / v_rho0;
            let t16 = v_tau0 * t15;
            let t17 = v_rho0 - v_rho1;
            let t18 = f64x8::splat(1.0) / t4;
            let t19 = t17 * t18;
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = t20 / f64x8::splat(2.0);
            let t22 = (simd::cbrt(t21));
            let t23 = t22 * t22;
            let t24 = t23 * t21;
            let t26 = (simd::cbrt(v_rho1));
            let t27 = t26 * t26;
            let t29 = f64x8::splat(1.0) / t27 / v_rho1;
            let t30 = v_tau1 * t29;
            let t31 = f64x8::splat(1.0) - t19;
            let t32 = t31 / f64x8::splat(2.0);
            let t33 = (simd::cbrt(t32));
            let t34 = t33 * t33;
            let t35 = t34 * t32;
            let t37 = t16 * t24 + t30 * t35;
            let t38 = f64x8::splat(1.0) / t37;
            let t39 = t17 * t17;
            let t40 = t38 * t39;
            let t43 = f64x8::splat(1.0) - t11 * t40 / f64x8::splat(8.0);
            let t44 = f64x8::splat(M_CBRT3);
            let t45 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t46 = (simd::cbrt(t45));
            let t47 = t44 * t46;
            let t48 = f64x8::splat(M_CBRT4);
            let t49 = t48 * t48;
            let t52 = t47 * t49 / t7;
            let t54 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t52;
            let t55 = ((t52).sqrt());
            let t58 = ((t52) * (t52).sqrt());
            let t60 = t44 * t44;
            let t61 = t46 * t46;
            let t62 = t60 * t61;
            let t65 = t62 * t48 / t8;
            let t67 = f64x8::splat(3.79785) * t55 + f64x8::splat(0.8969) * t52 + f64x8::splat(0.204775) * t58 + f64x8::splat(0.123235) * t65;
            let t70 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t67;
            let t71 = (simd::ln(t70));
            let t73 = f64x8::splat(0.0621814) * t54 * t71;
            let t74 = t39 * t39;
            let t75 = f64x8::splat(1.0) / t6;
            let t76 = t74 * t75;
            let t77 = (t20).simd_le(zeta_threshold);
            let t78 = (simd::cbrt(zeta_threshold));
            let t79 = t78 * zeta_threshold;
            let t80 = (simd::cbrt(t20));
            let t82 = ((t77).select(t79, t80 * t20));
            let t83 = (t31).simd_le(zeta_threshold);
            let t84 = (simd::cbrt(t31));
            let t86 = ((t83).select(t79, t84 * t31));
            let t87 = t82 + t86 - f64x8::splat(2.0);
            let t88 = f64x8::splat(M_CBRT2);
            let t91 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t88 - f64x8::splat(2.0));
            let t92 = t87 * t91;
            let t94 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t52;
            let t99 = f64x8::splat(7.05945) * t55 + f64x8::splat(1.549425) * t52 + f64x8::splat(0.420775) * t58 + f64x8::splat(0.1562925) * t65;
            let t102 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t99;
            let t103 = (simd::ln(t102));
            let t107 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t52;
            let t112 = f64x8::splat(5.1785) * t55 + f64x8::splat(0.905775) * t52 + f64x8::splat(0.1100325) * t58 + f64x8::splat(0.1241775) * t65;
            let t115 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t112;
            let t116 = (simd::ln(t115));
            let t117 = t107 * t116;
            let t119 = -f64x8::splat(0.0310907) * t94 * t103 + t73 - f64x8::splat(0.0197516734986138) * t117;
            let t120 = t92 * t119;
            let t124 = -t73 + t76 * t120 + f64x8::splat(0.0197516734986138) * t92 * t117;
            let tzk0 = t43 * t124;
            acc_zk = tzk0;
            let t125 = t6 * t4;
            let t127 = f64x8::splat(1.0) / t8 / t125;
            let t128 = t3 * t127;
            let t130 = f64x8::splat(7.0) / f64x8::splat(12.0) * t128 * t40;
            let t131 = t37 * t37;
            let t132 = f64x8::splat(1.0) / t131;
            let t133 = t132 * t39;
            let t134 = v_rho0 * v_rho0;
            let t136 = f64x8::splat(1.0) / t13 / t134;
            let t137 = v_tau0 * t136;
            let t139 = f64x8::splat(1.0) / t5;
            let t140 = t17 * t139;
            let t141 = t18 - t140;
            let t142 = t141 / f64x8::splat(2.0);
            let t143 = t23 * t142;
            let t145 = -t142;
            let t146 = t34 * t145;
            let t149 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t137 * t24 + f64x8::splat(5.0) / f64x8::splat(3.0) * t16 * t143 + f64x8::splat(5.0) / f64x8::splat(3.0) * t30 * t146;
            let t150 = t133 * t149;
            let t153 = t38 * t17;
            let t155 = t11 * t153 / f64x8::splat(4.0);
            let t156 = t130 + t11 * t150 / f64x8::splat(8.0) - t155;
            let t157 = t4 * t156;
            let t159 = t4 * t43;
            let t161 = f64x8::splat(1.0) / t7 / t4;
            let t162 = t49 * t161;
            let t165 = f64x8::splat(0.0011073470983333333) * t47 * t162 * t71;
            let t166 = t67 * t67;
            let t167 = f64x8::splat(1.0) / t166;
            let t168 = t54 * t167;
            let t170 = f64x8::splat(1.0) / t55 * t44;
            let t171 = t46 * t49;
            let t172 = t171 * t161;
            let t173 = t170 * t172;
            let t175 = t47 * t162;
            let t177 = ((t52).sqrt());
            let t178 = t177 * t44;
            let t179 = t178 * t172;
            let t184 = t62 * t48 / t8 / t4;
            let t186 = -f64x8::splat(0.632975) * t173 - f64x8::splat(0.29896666666666666) * t175 - f64x8::splat(0.1023875) * t179 - f64x8::splat(0.08215666666666667) * t184;
            let t187 = f64x8::splat(1.0) / t70;
            let t188 = t186 * t187;
            let t190 = f64x8::splat(1.0) * t168 * t188;
            let t191 = t39 * t17;
            let t192 = t191 * t75;
            let t194 = f64x8::splat(4.0) * t192 * t120;
            let t195 = f64x8::splat(1.0) / t125;
            let t196 = t74 * t195;
            let t198 = f64x8::splat(4.0) * t196 * t120;
            let t201 = ((t77).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t80 * t141));
            let t202 = -t141;
            let t205 = ((t83).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t84 * t202));
            let t207 = (t201 + t205) * t91;
            let t208 = t207 * t119;
            let t213 = t99 * t99;
            let t214 = f64x8::splat(1.0) / t213;
            let t215 = t94 * t214;
            let t220 = -f64x8::splat(1.176575) * t173 - f64x8::splat(0.516475) * t175 - f64x8::splat(0.2103875) * t179 - f64x8::splat(0.104195) * t184;
            let t221 = f64x8::splat(1.0) / t102;
            let t222 = t220 * t221;
            let t228 = t112 * t112;
            let t229 = f64x8::splat(1.0) / t228;
            let t230 = t107 * t229;
            let t235 = -f64x8::splat(0.8630833333333333) * t173 - f64x8::splat(0.301925) * t175 - f64x8::splat(0.05501625) * t179 - f64x8::splat(0.082785) * t184;
            let t236 = f64x8::splat(1.0) / t115;
            let t237 = t235 * t236;
            let t240 = f64x8::splat(0.0005323764196666666) * t47 * t162 * t103 + f64x8::splat(1.0) * t215 * t222 - t165 - t190 + f64x8::splat(0.00018311447306006544) * t47 * t162 * t116 + f64x8::splat(0.5848223622634646) * t230 * t237;
            let t241 = t92 * t240;
            let t242 = t76 * t241;
            let t245 = t92 * t44;
            let t247 = t171 * t161 * t116;
            let t249 = f64x8::splat(0.00018311447306006544) * t245 * t247;
            let t250 = t92 * t107;
            let t252 = t229 * t235 * t236;
            let t254 = f64x8::splat(0.5848223622634646) * t250 * t252;
            let t255 = t165 + t190 + t194 - t198 + t76 * t208 + t242 + f64x8::splat(0.0197516734986138) * t207 * t117 - t249 - t254;
            let tvrho0 = t157 * t124 + t159 * t255 + tzk0;
            acc_vrho_0 = tvrho0;
            let t257 = -t18 - t140;
            let t258 = t257 / f64x8::splat(2.0);
            let t259 = t23 * t258;
            let t261 = v_rho1 * v_rho1;
            let t263 = f64x8::splat(1.0) / t27 / t261;
            let t264 = v_tau1 * t263;
            let t266 = -t258;
            let t267 = t34 * t266;
            let t270 = f64x8::splat(5.0) / f64x8::splat(3.0) * t16 * t259 - f64x8::splat(5.0) / f64x8::splat(3.0) * t264 * t35 + f64x8::splat(5.0) / f64x8::splat(3.0) * t30 * t267;
            let t271 = t133 * t270;
            let t274 = t130 + t11 * t271 / f64x8::splat(8.0) + t155;
            let t275 = t4 * t274;
            let t279 = ((t77).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t80 * t257));
            let t280 = -t257;
            let t283 = ((t83).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t84 * t280));
            let t285 = (t279 + t283) * t91;
            let t286 = t285 * t119;
            let t290 = t165 + t190 - t194 - t198 + t76 * t286 + t242 + f64x8::splat(0.0197516734986138) * t285 * t117 - t249 - t254;
            let tvrho1 = t275 * t124 + t159 * t290 + tzk0;
            acc_vrho_1 = tvrho1;
            let t292 = t5 * t4;
            let t294 = f64x8::splat(1.0) / t8 / t292;
            let t295 = t294 * t38;
            let t296 = t39 * t124;
            let t297 = t295 * t296;
            let tvsigma0 = -t297 / f64x8::splat(8.0);
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = -t297 / f64x8::splat(4.0);
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = tvsigma0;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t300 = t294 * t3;
            let t301 = t300 * t132;
            let t302 = t39 * t15;
            let t303 = t24 * t124;
            let t304 = t302 * t303;
            let tvtau0 = t301 * t304 / f64x8::splat(8.0);
            acc_vtau_0 = tvtau0;
            let t306 = t39 * t29;
            let t307 = t35 * t124;
            let t308 = t306 * t307;
            let tvtau1 = t301 * t308 / f64x8::splat(8.0);
            acc_vtau_1 = tvtau1;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}
