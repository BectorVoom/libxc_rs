//! MGGA_K_RDA fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_rda.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_k_rda_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    param_A0: f64,
    param_A1: f64,
    param_A2: f64,
    param_A3: f64,
    param_a: f64,
    param_b: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_beta3: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_A0 = f64x8::splat(param_A0);
    let param_A1 = f64x8::splat(param_A1);
    let param_A2 = f64x8::splat(param_A2);
    let param_A3 = f64x8::splat(param_A3);
    let param_a = f64x8::splat(param_a);
    let param_b = f64x8::splat(param_b);
    let param_beta1 = f64x8::splat(param_beta1);
    let param_beta2 = f64x8::splat(param_beta2);
    let param_beta3 = f64x8::splat(param_beta3);
    let param_c = f64x8::splat(param_c);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2rholapl = V_ZERO;
        let mut acc_v2rhotau = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v2sigmalapl = V_ZERO;
        let mut acc_v2sigmatau = V_ZERO;
        let mut acc_v2lapl2 = V_ZERO;
        let mut acc_v2lapltau = V_ZERO;
        let mut acc_v2tau2 = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = t4 * t4;
            let t6 = f64x8::splat(M_CBRTPI);
            let t8 = t5 * t6 * f64x8::splat(M_PI);
            let t9 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t10 = zeta_threshold - f64x8::splat(1.0);
            let t12 = ((t9).select(t10, (t9).select(-t10, f64x8::splat(0.0))));
            let t13 = f64x8::splat(1.0) + t12;
            let t15 = (simd::cbrt(zeta_threshold));
            let t16 = t15 * t15;
            let t18 = (simd::cbrt(t13));
            let t19 = t18 * t18;
            let t21 = (((t13).simd_le(zeta_threshold)).select(t16 * zeta_threshold, t19 * t13));
            let t22 = (simd::cbrt(v_rho));
            let t23 = t22 * t22;
            let t24 = t21 * t23;
            let t25 = f64x8::splat(M_CBRT6);
            let t26 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t27 = (simd::cbrt(t26));
            let t28 = t27 * t27;
            let t29 = f64x8::splat(1.0) / t28;
            let t30 = t25 * t29;
            let t31 = f64x8::splat(M_CBRT2);
            let t32 = t31 * t31;
            let t33 = v_sigma * t32;
            let t34 = v_rho * v_rho;
            let t36 = f64x8::splat(1.0) / t23 / t34;
            let t38 = t30 * t33 * t36;
            let t40 = t25 * t25;
            let t42 = f64x8::splat(1.0) / t27 / t26;
            let t43 = t40 * t42;
            let t44 = v_sigma * v_sigma;
            let t45 = t44 * t31;
            let t46 = t34 * t34;
            let t47 = t46 * v_rho;
            let t49 = f64x8::splat(1.0) / t22 / t47;
            let t51 = t43 * t45 * t49;
            let t52 = param_a * t40;
            let t53 = t52 * t42;
            let t54 = v_lapl * v_lapl;
            let t55 = t54 * t31;
            let t56 = t34 * v_rho;
            let t58 = f64x8::splat(1.0) / t22 / t56;
            let t59 = t55 * t58;
            let t62 = f64x8::splat(2.0) * t53 * t59 + f64x8::splat(2.0) * t51;
            let t64 = ((t62).sqrt());
            let t67 = f64x8::splat(1.0) + param_beta1 * t64 / f64x8::splat(24.0);
            let t68 = t67 * t67;
            let t69 = f64x8::splat(1.0) / t68;
            let t72 = param_b * t40;
            let t73 = t72 * t42;
            let t76 = f64x8::splat(2.0) * t73 * t59 + f64x8::splat(2.0) * t51;
            let t77 = t76 * t76;
            let t79 = ((t76).sqrt());
            let t82 = f64x8::splat(1.0) + param_beta2 * t79 / f64x8::splat(24.0);
            let t83 = t82 * t82;
            let t84 = t83 * t83;
            let t85 = f64x8::splat(1.0) / t84;
            let t88 = param_c * t25;
            let t89 = t88 * t29;
            let t90 = v_lapl * t32;
            let t92 = f64x8::splat(1.0) / t23 / v_rho;
            let t96 = t89 * t90 * t92 / f64x8::splat(24.0) + t38 / f64x8::splat(24.0);
            let t97 = param_A3 * t96;
            let t99 = param_beta3 * t96 + f64x8::splat(1.0);
            let t100 = f64x8::splat(1.0) / t99;
            let t102 = f64x8::splat(5.0) / f64x8::splat(72.0) * t38 + param_A0 + param_A1 * t62 * t69 / f64x8::splat(576.0) + param_A2 * t77 * t85 / f64x8::splat(331776.0) + t97 * t100;
            let t106 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t102));
            let tzk0 = f64x8::splat(2.0) * t106;
            acc_zk = tzk0;
            let t108 = t21 / t22;
            let t113 = f64x8::splat(1.0) / t23 / t56;
            let t115 = t30 * t33 * t113;
            let t117 = t46 * t34;
            let t119 = f64x8::splat(1.0) / t22 / t117;
            let t122 = f64x8::splat(32.0) / f64x8::splat(3.0) * t43 * t45 * t119;
            let t124 = f64x8::splat(1.0) / t22 / t46;
            let t125 = t55 * t124;
            let t128 = -t122 - f64x8::splat(20.0) / f64x8::splat(3.0) * t53 * t125;
            let t132 = param_A1 * t64;
            let t134 = f64x8::splat(1.0) / t68 / t67;
            let t135 = t134 * param_beta1;
            let t139 = param_A2 * t76;
            let t142 = -t122 - f64x8::splat(20.0) / f64x8::splat(3.0) * t73 * t125;
            let t146 = t79 * t76;
            let t147 = param_A2 * t146;
            let t149 = f64x8::splat(1.0) / t84 / t82;
            let t150 = t149 * param_beta2;
            let t158 = -t115 / f64x8::splat(9.0) - f64x8::splat(5.0) / f64x8::splat(72.0) * t89 * t90 * t36;
            let t159 = param_A3 * t158;
            let t161 = t99 * t99;
            let t162 = f64x8::splat(1.0) / t161;
            let t163 = t162 * param_beta3;
            let t164 = t163 * t158;
            let t166 = -f64x8::splat(5.0) / f64x8::splat(27.0) * t115 + param_A1 * t128 * t69 / f64x8::splat(576.0) - t132 * t135 * t128 / f64x8::splat(13824.0) + t139 * t85 * t142 / f64x8::splat(165888.0) - t147 * t150 * t142 / f64x8::splat(3981312.0) + t159 * t100 - t97 * t164;
            let t171 = ((t3).select(f64x8::splat(0.0), t8 * t108 * t102 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t166));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t171 + f64x8::splat(2.0) * t106;
            acc_vrho = tvrho0;
            let t174 = t32 * t36;
            let t175 = t30 * t174;
            let t177 = param_A1 * t40;
            let t178 = t177 * t42;
            let t179 = v_sigma * t31;
            let t180 = t49 * t69;
            let t184 = t132 * t135;
            let t185 = t179 * t49;
            let t186 = t43 * t185;
            let t189 = t85 * t40;
            let t190 = t139 * t189;
            let t191 = t42 * v_sigma;
            let t192 = t31 * t49;
            let t193 = t191 * t192;
            let t196 = t147 * t150;
            let t199 = param_A3 * t25;
            let t200 = t199 * t29;
            let t204 = t97 * t163;
            let t207 = f64x8::splat(5.0) / f64x8::splat(72.0) * t175 + t178 * t179 * t180 / f64x8::splat(144.0) - t184 * t186 / f64x8::splat(3456.0) + t190 * t193 / f64x8::splat(41472.0) - t196 * t186 / f64x8::splat(995328.0) + t200 * t174 * t100 / f64x8::splat(24.0) - t204 * t175 / f64x8::splat(24.0);
            let t211 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t207));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t211;
            acc_vsigma = tvsigma0;
            let t213 = param_A1 * param_a;
            let t214 = t213 * t43;
            let t215 = v_lapl * t31;
            let t216 = t58 * t69;
            let t220 = t135 * param_a;
            let t221 = t132 * t220;
            let t222 = t215 * t58;
            let t223 = t43 * t222;
            let t226 = t85 * param_b;
            let t227 = t139 * t226;
            let t231 = t147 * t150 * param_b;
            let t234 = param_A3 * param_c;
            let t235 = t234 * t25;
            let t236 = t29 * t32;
            let t242 = t88 * t236 * t92;
            let t245 = t214 * t215 * t216 / f64x8::splat(144.0) - t221 * t223 / f64x8::splat(3456.0) + t227 * t223 / f64x8::splat(41472.0) - t231 * t223 / f64x8::splat(995328.0) + t235 * t236 * t92 * t100 / f64x8::splat(24.0) - t204 * t242 / f64x8::splat(24.0);
            let t249 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t245));
            let tvlapl0 = f64x8::splat(2.0) * v_rho * t249;
            acc_vlapl = tvlapl0;
            let tvtau0 = f64x8::splat(0.0);
            acc_vtau = tvtau0;
            let t254 = t21 / t22 / v_rho;
            let t262 = f64x8::splat(1.0) / t23 / t46;
            let t264 = t30 * t33 * t262;
            let t266 = t46 * t56;
            let t268 = f64x8::splat(1.0) / t22 / t266;
            let t271 = f64x8::splat(608.0) / f64x8::splat(9.0) * t43 * t45 * t268;
            let t272 = t55 * t49;
            let t275 = t271 + f64x8::splat(260.0) / f64x8::splat(9.0) * t53 * t272;
            let t276 = param_A1 * t275;
            let t279 = t128 * t128;
            let t280 = param_A1 * t279;
            let t281 = f64x8::splat(1.0) / t64;
            let t282 = t135 * t281;
            let t285 = t68 * t68;
            let t286 = f64x8::splat(1.0) / t285;
            let t287 = param_A1 * t286;
            let t288 = param_beta1 * param_beta1;
            let t295 = t142 * t142;
            let t296 = param_A2 * t295;
            let t299 = param_A2 * t79;
            let t306 = t271 + f64x8::splat(260.0) / f64x8::splat(9.0) * t73 * t272;
            let t307 = t85 * t306;
            let t311 = f64x8::splat(1.0) / t84 / t83;
            let t312 = param_beta2 * param_beta2;
            let t313 = t311 * t312;
            let t324 = f64x8::splat(11.0) / f64x8::splat(27.0) * t264 + f64x8::splat(5.0) / f64x8::splat(27.0) * t89 * t90 * t113;
            let t325 = param_A3 * t324;
            let t327 = t158 * t158;
            let t328 = param_A3 * t327;
            let t332 = f64x8::splat(1.0) / t161 / t99;
            let t333 = param_beta3 * param_beta3;
            let t334 = t332 * t333;
            let t335 = t334 * t327;
            let t338 = t163 * t324;
            let t340 = f64x8::splat(55.0) / f64x8::splat(81.0) * t264 + t276 * t69 / f64x8::splat(576.0) - t280 * t282 / f64x8::splat(9216.0) + t287 * t288 * t279 / f64x8::splat(221184.0) - t132 * t135 * t275 / f64x8::splat(13824.0) + t296 * t85 / f64x8::splat(165888.0) - f64x8::splat(7.0) / f64x8::splat(7962624.0) * t299 * t149 * t295 * param_beta2 + t139 * t307 / f64x8::splat(165888.0) + f64x8::splat(5.0) / f64x8::splat(191102976.0) * t139 * t313 * t295 - t147 * t150 * t306 / f64x8::splat(3981312.0) + t325 * t100 - f64x8::splat(2.0) * t328 * t163 + f64x8::splat(2.0) * t97 * t335 - t97 * t338;
            let t345 = ((t3).select(f64x8::splat(0.0), -t8 * t254 * t102 / f64x8::splat(30.0) + t8 * t108 * t166 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t340));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t345 + f64x8::splat(4.0) * t171;
            acc_v2rho2 = tv2rho20;
            let t351 = t32 * t113;
            let t352 = t30 * t351;
            let t354 = t119 * t69;
            let t358 = t191 * t31;
            let t359 = t177 * t358;
            let t360 = t49 * t134;
            let t361 = param_beta1 * t281;
            let t362 = t361 * t128;
            let t363 = t360 * t362;
            let t367 = t287 * t288 * t40;
            let t368 = t192 * t128;
            let t373 = t43 * t179 * t119;
            let t376 = param_A2 * t142;
            let t377 = t376 * t189;
            let t380 = t149 * t40;
            let t381 = t380 * t42;
            let t382 = t299 * t381;
            let t383 = t49 * param_beta2;
            let t384 = t383 * t142;
            let t388 = t31 * t119;
            let t389 = t191 * t388;
            let t393 = t139 * t313 * t40;
            let t394 = t192 * t142;
            let t403 = t199 * t236;
            let t404 = t36 * t162;
            let t405 = param_beta3 * t158;
            let t409 = t97 * t334;
            let t411 = t30 * t174 * t158;
            let t416 = -f64x8::splat(5.0) / f64x8::splat(27.0) * t352 - t178 * t179 * t354 / f64x8::splat(27.0) - t359 * t363 / f64x8::splat(2304.0) + t367 * t191 * t368 / f64x8::splat(55296.0) + t184 * t373 / f64x8::splat(648.0) + t377 * t193 / f64x8::splat(41472.0) - f64x8::splat(7.0) / f64x8::splat(1990656.0) * t382 * t179 * t384 - t190 * t389 / f64x8::splat(7776.0) + f64x8::splat(5.0) / f64x8::splat(47775744.0) * t393 * t191 * t394 + t196 * t373 / f64x8::splat(186624.0) - t200 * t351 * t100 / f64x8::splat(9.0) - t403 * t404 * t405 / f64x8::splat(12.0) + t409 * t411 / f64x8::splat(12.0) + t204 * t352 / f64x8::splat(9.0);
            let t421 = ((t3).select(f64x8::splat(0.0), t8 * t108 * t207 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t416));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t421 + f64x8::splat(2.0) * t211;
            acc_v2rhosigma = tv2rhosigma0;
            let t427 = t124 * t69;
            let t431 = t43 * v_lapl;
            let t432 = t213 * t431;
            let t433 = t31 * t58;
            let t434 = t433 * t134;
            let t438 = t288 * param_a;
            let t440 = t287 * t438 * t40;
            let t441 = t42 * v_lapl;
            let t442 = t433 * t128;
            let t447 = t43 * t215 * t124;
            let t450 = t376 * t226;
            let t453 = t149 * param_b;
            let t454 = t453 * t40;
            let t455 = t299 * t454;
            let t456 = t441 * t31;
            let t457 = t58 * param_beta2;
            let t458 = t457 * t142;
            let t465 = t139 * t313 * param_b;
            let t466 = t433 * t142;
            let t476 = t234 * t30;
            let t477 = t32 * t92;
            let t482 = t97 * t334 * param_c;
            let t491 = -f64x8::splat(5.0) / f64x8::splat(216.0) * t214 * t215 * t427 - t432 * t434 * t362 / f64x8::splat(2304.0) + t440 * t441 * t442 / f64x8::splat(55296.0) + f64x8::splat(5.0) / f64x8::splat(5184.0) * t221 * t447 + t450 * t223 / f64x8::splat(41472.0) - f64x8::splat(7.0) / f64x8::splat(1990656.0) * t455 * t456 * t458 - f64x8::splat(5.0) / f64x8::splat(62208.0) * t227 * t447 + f64x8::splat(5.0) / f64x8::splat(47775744.0) * t465 * t431 * t466 + f64x8::splat(5.0) / f64x8::splat(1492992.0) * t231 * t447 - f64x8::splat(5.0) / f64x8::splat(72.0) * t235 * t236 * t36 * t100 - t476 * t477 * t164 / f64x8::splat(12.0) + t482 * t30 * t477 * t158 / f64x8::splat(12.0) + f64x8::splat(5.0) / f64x8::splat(72.0) * t204 * t88 * t236 * t36;
            let t496 = ((t3).select(f64x8::splat(0.0), t8 * t108 * t245 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t491));
            let tv2rholapl0 = f64x8::splat(2.0) * v_rho * t496 + f64x8::splat(2.0) * t249;
            acc_v2rholapl = tv2rholapl0;
            let tv2rhotau0 = f64x8::splat(0.0);
            acc_v2rhotau = tv2rhotau0;
            let t502 = param_A1 * t25;
            let t503 = t26 * t26;
            let t505 = f64x8::splat(1.0) / t28 / t503;
            let t506 = t505 * t44;
            let t507 = t502 * t506;
            let t508 = t46 * t46;
            let t509 = t508 * t34;
            let t511 = f64x8::splat(1.0) / t23 / t509;
            let t512 = t32 * t511;
            let t517 = t287 * t288 * t25;
            let t521 = t43 * t192;
            let t524 = param_A2 * t25;
            let t525 = t524 * t505;
            let t526 = t44 * t32;
            let t527 = t511 * t85;
            let t532 = t299 * t149 * t25;
            let t533 = t512 * param_beta2;
            let t537 = t139 * t85;
            let t540 = t139 * t313;
            let t541 = t25 * t505;
            let t548 = param_A3 * t40;
            let t549 = t548 * t42;
            let t550 = t192 * t163;
            let t555 = t178 * t192 * t69 / f64x8::splat(144.0) - t507 * t512 * t282 / f64x8::splat(96.0) + t517 * t506 * t512 / f64x8::splat(2304.0) - t184 * t521 / f64x8::splat(3456.0) + t525 * t526 * t527 / f64x8::splat(1728.0) - f64x8::splat(7.0) / f64x8::splat(82944.0) * t532 * t506 * t533 + t537 * t521 / f64x8::splat(41472.0) + f64x8::splat(5.0) / f64x8::splat(1990656.0) * t540 * t541 * t526 * t511 - t196 * t521 / f64x8::splat(995328.0) - t549 * t550 / f64x8::splat(144.0) + t409 * t521 / f64x8::splat(144.0);
            let t559 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t555));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t559;
            acc_v2sigma2 = tv2sigma20;
            let t561 = t541 * v_lapl;
            let t562 = t213 * t561;
            let t564 = f64x8::splat(1.0) / t23 / t508;
            let t565 = t32 * t564;
            let t566 = t565 * t134;
            let t567 = t361 * v_sigma;
            let t572 = t287 * t438 * t25;
            let t573 = t505 * v_lapl;
            let t574 = t565 * v_sigma;
            let t578 = t505 * v_sigma;
            let t579 = t524 * t578;
            let t580 = t226 * v_lapl;
            let t585 = t299 * t453 * t25;
            let t586 = t573 * t32;
            let t588 = t564 * param_beta2 * v_sigma;
            let t595 = t234 * t43;
            let t596 = t31 * t124;
            let t597 = t596 * t163;
            let t600 = param_c * t40;
            let t601 = t42 * t31;
            let t602 = t601 * t124;
            let t606 = -t562 * t566 * t567 / f64x8::splat(96.0) + t572 * t573 * t574 / f64x8::splat(2304.0) + t579 * t565 * t580 / f64x8::splat(1728.0) - f64x8::splat(7.0) / f64x8::splat(82944.0) * t585 * t586 * t588 + f64x8::splat(5.0) / f64x8::splat(1990656.0) * t465 * t561 * t574 - t595 * t597 / f64x8::splat(144.0) + t409 * t600 * t602 / f64x8::splat(144.0);
            let t610 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t606));
            let tv2sigmalapl0 = f64x8::splat(2.0) * v_rho * t610;
            acc_v2sigmalapl = tv2sigmalapl0;
            let tv2sigmatau0 = f64x8::splat(0.0);
            acc_v2sigmatau = tv2sigmatau0;
            let t612 = t213 * t40;
            let t616 = param_a * param_a;
            let t617 = param_A1 * t616;
            let t618 = t541 * t54;
            let t619 = t617 * t618;
            let t621 = f64x8::splat(1.0) / t23 / t117;
            let t622 = t32 * t621;
            let t623 = t622 * t282;
            let t627 = t287 * t288 * t616;
            let t628 = t54 * t32;
            let t629 = t628 * t621;
            let t630 = t541 * t629;
            let t633 = t601 * t58;
            let t637 = param_b * param_b;
            let t638 = param_A2 * t637;
            let t639 = t638 * t541;
            let t640 = t621 * t85;
            let t644 = t149 * t637;
            let t646 = t299 * t644 * t25;
            let t647 = t505 * t54;
            let t648 = t622 * param_beta2;
            let t652 = t43 * t433;
            let t656 = t139 * t313 * t637;
            let t659 = t72 * t633;
            let t662 = param_c * param_c;
            let t663 = param_A3 * t662;
            let t664 = t663 * t43;
            let t668 = t662 * t40;
            let t672 = t612 * t601 * t216 / f64x8::splat(144.0) - t619 * t623 / f64x8::splat(96.0) + t627 * t630 / f64x8::splat(2304.0) - t184 * t52 * t633 / f64x8::splat(3456.0) + t639 * t628 * t640 / f64x8::splat(1728.0) - f64x8::splat(7.0) / f64x8::splat(82944.0) * t646 * t647 * t648 + t227 * t652 / f64x8::splat(41472.0) + f64x8::splat(5.0) / f64x8::splat(1990656.0) * t656 * t630 - t196 * t659 / f64x8::splat(995328.0) - t664 * t433 * t163 / f64x8::splat(144.0) + t409 * t668 * t633 / f64x8::splat(144.0);
            let t676 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t672));
            let tv2lapl20 = f64x8::splat(2.0) * v_rho * t676;
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let tv2tau20 = f64x8::splat(0.0);
            acc_v2tau2 = tv2tau20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rholapl.into(); v2rholapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhotau.into(); v2rhotau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigmalapl.into(); v2sigmalapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigmatau.into(); v2sigmatau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2lapl2.into(); v2lapl2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2lapltau.into(); v2lapltau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2tau2.into(); v2tau2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
