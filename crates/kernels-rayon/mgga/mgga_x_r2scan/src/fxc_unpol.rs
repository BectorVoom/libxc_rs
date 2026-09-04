//! MGGA_X_R2SCAN fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_r2scan.c`
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
pub fn mgga_x_r2scan_fxc_unpol(
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
    param_c1: f64,
    param_c2: f64,
    param_d: f64,
    param_dp2: f64,
    param_eta: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c1 = f64x8::splat(param_c1);
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_dp2 = f64x8::splat(param_dp2);
    let param_eta = f64x8::splat(param_eta);
    let param_k1 = f64x8::splat(param_k1);
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
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = t7 * t18;
            let t20 = (simd::cbrt(v_rho));
            let t22 = f64x8::splat(20.0) / f64x8::splat(27.0) + f64x8::splat(5.0) / f64x8::splat(3.0) * param_eta;
            let t23 = f64x8::splat(M_CBRT6);
            let t24 = t23 * t23;
            let t25 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t26 = (simd::cbrt(t25));
            let t27 = t26 * t25;
            let t28 = f64x8::splat(1.0) / t27;
            let t29 = t24 * t28;
            let t30 = v_sigma * v_sigma;
            let t32 = f64x8::splat(M_CBRT2);
            let t33 = v_rho * v_rho;
            let t34 = t33 * t33;
            let t35 = t34 * v_rho;
            let t37 = f64x8::splat(1.0) / t20 / t35;
            let t38 = t32 * t37;
            let t39 = param_dp2 * param_dp2;
            let t40 = t39 * t39;
            let t41 = f64x8::splat(1.0) / t40;
            let t45 = (simd::exp(-t29 * t30 * t38 * t41 / f64x8::splat(288.0)));
            let t49 = (-f64x8::splat(0.162742215233874) * t22 * t45 + f64x8::splat(10.0) / f64x8::splat(81.0)) * t23;
            let t50 = t26 * t26;
            let t51 = f64x8::splat(1.0) / t50;
            let t52 = t49 * t51;
            let t53 = t32 * t32;
            let t54 = v_sigma * t53;
            let t55 = t20 * t20;
            let t57 = f64x8::splat(1.0) / t55 / t33;
            let t58 = t54 * t57;
            let t61 = param_k1 + t52 * t58 / f64x8::splat(24.0);
            let t65 = param_k1 * (f64x8::splat(1.0) - param_k1 / t61);
            let t66 = v_tau * t53;
            let t67 = t55 * v_rho;
            let t68 = f64x8::splat(1.0) / t67;
            let t71 = t66 * t68 - t58 / f64x8::splat(8.0);
            let t75 = t53 * t57;
            let t78 = f64x8::splat(3.0) / f64x8::splat(10.0) * t24 * t50 + param_eta * v_sigma * t75 / f64x8::splat(8.0);
            let t79 = f64x8::splat(1.0) / t78;
            let t80 = t71 * t79;
            let t81 = (t80).simd_le(f64x8::splat(0.0));
            let t82 = (f64x8::splat(0.0)).simd_lt(t80);
            let t83 = ((t82).select(f64x8::splat(0.0), t80));
            let t84 = param_c1 * t83;
            let t85 = f64x8::splat(1.0) - t83;
            let t86 = f64x8::splat(1.0) / t85;
            let t88 = (simd::exp(-t84 * t86));
            let t89 = (t80).simd_le(f64x8::splat(2.5));
            let t90 = (f64x8::splat(2.5)).simd_lt(t80);
            let t91 = ((t90).select(f64x8::splat(2.5), t80));
            let t93 = t91 * t91;
            let t95 = t93 * t91;
            let t97 = t93 * t93;
            let t99 = t97 * t91;
            let t101 = t97 * t93;
            let t106 = ((t90).select(t80, f64x8::splat(2.5)));
            let t107 = f64x8::splat(1.0) - t106;
            let t110 = (simd::exp(param_c2 / t107));
            let t112 = ((t81).select(t88, (t89).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t91 - f64x8::splat(0.4445555) * t93 - f64x8::splat(0.663086601049) * t95 + f64x8::splat(1.45129704449) * t97 - f64x8::splat(0.887998041597) * t99 + f64x8::splat(0.234528941479) * t101 - f64x8::splat(0.023185843322) * t97 * t95, -param_d * t110)));
            let t113 = f64x8::splat(0.174) - t65;
            let t115 = t112 * t113 + t65 + f64x8::splat(1.0);
            let t117 = ((f64x8::splat(3.0)).sqrt());
            let t118 = f64x8::splat(1.0) / t26;
            let t119 = t24 * t118;
            let t120 = ((v_sigma).sqrt());
            let t121 = t120 * t32;
            let t123 = f64x8::splat(1.0) / t20 / v_rho;
            let t125 = t119 * t121 * t123;
            let t126 = ((t125).sqrt());
            let t130 = (simd::exp(-f64x8::splat(9.8958) * t117 / t126));
            let t131 = f64x8::splat(1.0) - t130;
            let t135 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t115 * t131));
            let tzk0 = f64x8::splat(2.0) * t135;
            acc_zk = tzk0;
            let t136 = f64x8::splat(1.0) / t55;
            let t141 = param_k1 * param_k1;
            let t142 = t61 * t61;
            let t143 = f64x8::splat(1.0) / t142;
            let t144 = t141 * t143;
            let t145 = t30 * v_sigma;
            let t146 = t22 * t145;
            let t147 = t34 * t34;
            let t148 = t147 * v_rho;
            let t149 = f64x8::splat(1.0) / t148;
            let t151 = t149 * t41 * t45;
            let t154 = t33 * v_rho;
            let t156 = f64x8::splat(1.0) / t55 / t154;
            let t157 = t54 * t156;
            let t160 = -f64x8::splat(1.5469524941471938e-05) * t146 * t151 - t52 * t157 / f64x8::splat(9.0);
            let t165 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t66 * t57 + t157 / f64x8::splat(3.0);
            let t167 = t78 * t78;
            let t168 = f64x8::splat(1.0) / t167;
            let t169 = t71 * t168;
            let t170 = t169 * param_eta;
            let t173 = t165 * t79 + t170 * t157 / f64x8::splat(3.0);
            let t174 = ((t82).select(f64x8::splat(0.0), t173));
            let t177 = t85 * t85;
            let t178 = f64x8::splat(1.0) / t177;
            let t179 = t178 * t174;
            let t181 = -param_c1 * t174 * t86 - t84 * t179;
            let t182 = t181 * t88;
            let t183 = ((t90).select(f64x8::splat(0.0), t173));
            let t185 = t91 * t183;
            let t187 = t93 * t183;
            let t189 = t95 * t183;
            let t191 = t97 * t183;
            let t193 = t99 * t183;
            let t198 = param_d * param_c2;
            let t199 = t107 * t107;
            let t200 = f64x8::splat(1.0) / t199;
            let t201 = ((t90).select(t173, f64x8::splat(0.0)));
            let t205 = ((t81).select(t182, (t89).select(-f64x8::splat(0.667) * t183 - f64x8::splat(0.889111) * t185 - f64x8::splat(1.989259803147) * t187 + f64x8::splat(5.80518817796) * t189 - f64x8::splat(4.439990207985) * t191 + f64x8::splat(1.407173648874) * t193 - f64x8::splat(0.162300903254) * t101 * t183, -t198 * t200 * t201 * t110)));
            let t207 = t112 * t141;
            let t208 = t143 * t160;
            let t210 = t205 * t113 + t144 * t160 - t207 * t208;
            let t215 = (simd::pow(f64x8::splat(3.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t216 = t215 * t215;
            let t217 = t216 * t216;
            let t219 = t217 * t215 * t18;
            let t220 = f64x8::splat(1.0) / t33;
            let t221 = t220 * t115;
            let t223 = f64x8::splat(1.0) / t126 / t125;
            let t225 = t219 * t221 * t223;
            let t226 = t121 * t130;
            let t227 = t119 * t226;
            let t231 = ((t3).select(f64x8::splat(0.0), -t19 * t136 * t115 * t131 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t210 * t131 - f64x8::splat(1.6891736332904388) * t225 * t227));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t231 + f64x8::splat(2.0) * t135;
            acc_vrho = tvrho0;
            let t234 = t22 * t30;
            let t235 = f64x8::splat(1.0) / t147;
            let t237 = t235 * t41 * t45;
            let t240 = t51 * t53;
            let t244 = f64x8::splat(5.801071853051976e-06) * t234 * t237 + t49 * t240 * t57 / f64x8::splat(24.0);
            let t246 = t75 * t79;
            let t247 = param_eta * t53;
            let t248 = t247 * t57;
            let t251 = -t169 * t248 / f64x8::splat(8.0) - t246 / f64x8::splat(8.0);
            let t252 = ((t82).select(f64x8::splat(0.0), t251));
            let t253 = param_c1 * t252;
            let t255 = t178 * t252;
            let t257 = -t253 * t86 - t84 * t255;
            let t258 = t257 * t88;
            let t259 = ((t90).select(f64x8::splat(0.0), t251));
            let t261 = t91 * t259;
            let t263 = t93 * t259;
            let t265 = t95 * t259;
            let t267 = t97 * t259;
            let t269 = t99 * t259;
            let t274 = ((t90).select(t251, f64x8::splat(0.0)));
            let t278 = ((t81).select(t258, (t89).select(-f64x8::splat(0.667) * t259 - f64x8::splat(0.889111) * t261 - f64x8::splat(1.989259803147) * t263 + f64x8::splat(5.80518817796) * t265 - f64x8::splat(4.439990207985) * t267 + f64x8::splat(1.407173648874) * t269 - f64x8::splat(0.162300903254) * t101 * t259, -t198 * t200 * t274 * t110)));
            let t280 = t143 * t244;
            let t282 = t278 * t113 + t144 * t244 - t207 * t280;
            let t287 = f64x8::splat(1.0) / v_rho;
            let t288 = t287 * t115;
            let t290 = t219 * t288 * t223;
            let t291 = f64x8::splat(1.0) / t120;
            let t293 = t291 * t32 * t130;
            let t294 = t119 * t293;
            let t298 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t282 * t131 + f64x8::splat(0.6334401124839145) * t290 * t294));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t298;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t301 = t53 * t68 * t79;
            let t302 = ((t82).select(f64x8::splat(0.0), t301));
            let t303 = param_c1 * t302;
            let t305 = t178 * t302;
            let t307 = -t303 * t86 - t84 * t305;
            let t308 = t307 * t88;
            let t309 = ((t90).select(f64x8::splat(0.0), t301));
            let t311 = t91 * t309;
            let t313 = t93 * t309;
            let t315 = t95 * t309;
            let t317 = t97 * t309;
            let t319 = t99 * t309;
            let t324 = ((t90).select(t301, f64x8::splat(0.0)));
            let t328 = ((t81).select(t308, (t89).select(-f64x8::splat(0.667) * t309 - f64x8::splat(0.889111) * t311 - f64x8::splat(1.989259803147) * t313 + f64x8::splat(5.80518817796) * t315 - f64x8::splat(4.439990207985) * t317 + f64x8::splat(1.407173648874) * t319 - f64x8::splat(0.162300903254) * t101 * t309, -t198 * t200 * t324 * t110)));
            let t329 = t20 * t328;
            let t330 = t113 * t131;
            let t334 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t329 * t330));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t334;
            acc_vtau = tvtau0;
            let t345 = f64x8::splat(1.0) / t154;
            let t346 = t345 * t115;
            let t348 = t219 * t346 * t223;
            let t352 = f64x8::splat(1.0) / t142 / t61;
            let t353 = t141 * t352;
            let t354 = t160 * t160;
            let t357 = t147 * t33;
            let t358 = f64x8::splat(1.0) / t357;
            let t360 = t358 * t41 * t45;
            let t363 = t30 * t30;
            let t364 = t363 * v_sigma;
            let t365 = t22 * t364;
            let t366 = t34 * t154;
            let t370 = t40 * t40;
            let t371 = f64x8::splat(1.0) / t370;
            let t372 = f64x8::splat(1.0) / t20 / t147 / t366 * t371;
            let t374 = t32 * t45;
            let t375 = t29 * t374;
            let t379 = f64x8::splat(1.0) / t55 / t34;
            let t380 = t54 * t379;
            let t383 = f64x8::splat(0.00018047779098383926) * t146 * t360 - f64x8::splat(2.864726841013322e-07) * t365 * t372 * t375 + f64x8::splat(11.0) / f64x8::splat(27.0) * t52 * t380;
            let t388 = f64x8::splat(40.0) / f64x8::splat(9.0) * t66 * t156 - f64x8::splat(11.0) / f64x8::splat(9.0) * t380;
            let t390 = t165 * t168;
            let t391 = t390 * param_eta;
            let t395 = f64x8::splat(1.0) / t167 / t78;
            let t396 = t71 * t395;
            let t397 = param_eta * param_eta;
            let t398 = t396 * t397;
            let t399 = t30 * t32;
            let t401 = f64x8::splat(1.0) / t20 / t366;
            let t402 = t399 * t401;
            let t407 = t388 * t79 + f64x8::splat(2.0) / f64x8::splat(3.0) * t391 * t157 + f64x8::splat(4.0) / f64x8::splat(9.0) * t398 * t402 - f64x8::splat(11.0) / f64x8::splat(9.0) * t170 * t380;
            let t408 = ((t82).select(f64x8::splat(0.0), t407));
            let t409 = param_c1 * t408;
            let t411 = t174 * t174;
            let t416 = f64x8::splat(1.0) / t177 / t85;
            let t417 = t416 * t411;
            let t420 = t178 * t408;
            let t422 = -f64x8::splat(2.0) * param_c1 * t411 * t178 - t409 * t86 - f64x8::splat(2.0) * t84 * t417 - t84 * t420;
            let t423 = t422 * t88;
            let t424 = t181 * t181;
            let t425 = t424 * t88;
            let t427 = ((t90).select(f64x8::splat(0.0), t407));
            let t429 = t183 * t183;
            let t433 = t91 * t429;
            let t437 = t93 * t429;
            let t441 = t95 * t429;
            let t445 = t97 * t429;
            let t453 = -f64x8::splat(0.667) * t427 - f64x8::splat(0.889111) * t429 - f64x8::splat(0.889111) * t91 * t427 - f64x8::splat(3.978519606294) * t433 - f64x8::splat(1.989259803147) * t93 * t427 + f64x8::splat(17.41556453388) * t437 + f64x8::splat(5.80518817796) * t95 * t427 - f64x8::splat(17.75996083194) * t441 - f64x8::splat(4.439990207985) * t97 * t427 + f64x8::splat(7.03586824437) * t445 + f64x8::splat(1.407173648874) * t99 * t427 - f64x8::splat(0.973805419524) * t99 * t429 - f64x8::splat(0.162300903254) * t101 * t427;
            let t454 = t199 * t107;
            let t455 = f64x8::splat(1.0) / t454;
            let t456 = t201 * t201;
            let t461 = ((t90).select(t407, f64x8::splat(0.0)));
            let t465 = param_c2 * param_c2;
            let t466 = param_d * t465;
            let t467 = t199 * t199;
            let t468 = f64x8::splat(1.0) / t467;
            let t473 = ((t81).select(t423 + t425, (t89).select(t453, -t198 * t200 * t461 * t110 - f64x8::splat(2.0) * t198 * t455 * t456 * t110 - t466 * t468 * t456 * t110)));
            let t475 = t205 * t141;
            let t478 = t352 * t354;
            let t481 = t143 * t383;
            let t483 = t473 * t113 + t144 * t383 + f64x8::splat(2.0) * t207 * t478 - t207 * t481 - f64x8::splat(2.0) * t475 * t208 - f64x8::splat(2.0) * t353 * t354;
            let t490 = t219 * t220 * t210 * t223;
            let t494 = f64x8::splat(1.0) / t20 / t34;
            let t495 = t494 * t115;
            let t496 = t23 * t51;
            let t500 = f64x8::splat(1.0) / t126 / t496 / t58 / f64x8::splat(6.0);
            let t502 = t219 * t495 * t500;
            let t503 = t54 * t130;
            let t504 = t496 * t503;
            let t507 = t4 * t18;
            let t508 = f64x8::splat(1.0) / t20;
            let t510 = t507 * t508 * t115;
            let t512 = t240 * t130;
            let t513 = t291 * t23 * t512;
            let t517 = ((t3).select(f64x8::splat(0.0), t19 * t68 * t115 * t131 / f64x8::splat(12.0) - t19 * t136 * t210 * t131 / f64x8::splat(4.0) + f64x8::splat(2.8152893888173978) * t348 * t227 - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t483 * t131 - f64x8::splat(3.3783472665808776) * t490 * t227 - f64x8::splat(20.270083599485265) * t502 * t504 + f64x8::splat(27.496264583922507) * t510 * t513));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t517 + f64x8::splat(4.0) * t231;
            acc_v2rho2 = tv2rho20;
            let t524 = t244 * t160;
            let t529 = t22 * t363;
            let t530 = t34 * t33;
            let t531 = t147 * t530;
            let t533 = f64x8::splat(1.0) / t20 / t531;
            let t534 = t533 * t371;
            let t541 = -f64x8::splat(6.187809976588775e-05) * t234 * t151 + f64x8::splat(1.0742725653799956e-07) * t529 * t534 * t375 - t49 * t240 * t156 / f64x8::splat(9.0);
            let t544 = t53 * t156 * t79;
            let t547 = f64x8::splat(1.0) / t20 / t530;
            let t548 = t32 * t547;
            let t549 = t168 * param_eta;
            let t550 = t549 * v_sigma;
            let t551 = t548 * t550;
            let t555 = t548 * v_sigma;
            let t558 = t247 * t156;
            let t561 = t544 / f64x8::splat(3.0) - t551 / f64x8::splat(12.0) - t390 * t248 / f64x8::splat(8.0) - t398 * t555 / f64x8::splat(6.0) + t169 * t558 / f64x8::splat(3.0);
            let t562 = ((t82).select(f64x8::splat(0.0), t561));
            let t563 = param_c1 * t562;
            let t567 = t416 * t252;
            let t568 = t567 * t174;
            let t571 = t178 * t562;
            let t573 = -f64x8::splat(2.0) * t253 * t179 - t563 * t86 - f64x8::splat(2.0) * t84 * t568 - t84 * t571;
            let t574 = t573 * t88;
            let t575 = t257 * t181;
            let t578 = ((t90).select(f64x8::splat(0.0), t561));
            let t580 = t183 * t259;
            let t582 = t91 * t578;
            let t586 = t93 * t578;
            let t590 = t95 * t578;
            let t594 = t97 * t578;
            let t598 = t99 * t578;
            let t604 = -f64x8::splat(0.667) * t578 - f64x8::splat(0.889111) * t580 - f64x8::splat(0.889111) * t582 - f64x8::splat(3.978519606294) * t261 * t183 - f64x8::splat(1.989259803147) * t586 + f64x8::splat(17.41556453388) * t263 * t183 + f64x8::splat(5.80518817796) * t590 - f64x8::splat(17.75996083194) * t265 * t183 - f64x8::splat(4.439990207985) * t594 + f64x8::splat(7.03586824437) * t267 * t183 + f64x8::splat(1.407173648874) * t598 - f64x8::splat(0.973805419524) * t269 * t183 - f64x8::splat(0.162300903254) * t101 * t578;
            let t605 = t198 * t455;
            let t606 = t274 * t110;
            let t607 = t606 * t201;
            let t610 = ((t90).select(t561, f64x8::splat(0.0)));
            let t614 = t466 * t468;
            let t617 = ((t81).select(t575 * t88 + t574, (t89).select(t604, -t198 * t200 * t610 * t110 - f64x8::splat(2.0) * t605 * t607 - t614 * t607)));
            let t619 = t278 * t141;
            let t622 = t352 * t244;
            let t623 = t622 * t160;
            let t626 = t143 * t541;
            let t628 = t617 * t113 + t144 * t541 + f64x8::splat(2.0) * t207 * t623 - t207 * t626 - t619 * t208 - t475 * t280 - f64x8::splat(2.0) * t353 * t524;
            let t635 = t219 * t220 * t282 * t223;
            let t642 = t219 * t287 * t210 * t223;
            let t646 = f64x8::splat(1.0) / t20 / t154;
            let t647 = t646 * t115;
            let t649 = t500 * t23;
            let t650 = t649 * t512;
            let t654 = t507 * t55 * t115;
            let t655 = t120 * v_sigma;
            let t656 = f64x8::splat(1.0) / t655;
            let t658 = t656 * t23 * t512;
            let t662 = ((t3).select(f64x8::splat(0.0), -t19 * t136 * t282 * t131 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t628 * t131 - f64x8::splat(1.6891736332904388) * t635 * t227 - f64x8::splat(0.6334401124839145) * t225 * t294 + f64x8::splat(0.6334401124839145) * t642 * t294 + f64x8::splat(7.601281349806975) * t219 * t647 * t650 - f64x8::splat(10.311099218970941) * t654 * t658));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t662 + f64x8::splat(2.0) * t298;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t665 = t136 * t328;
            let t672 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t246 + f64x8::splat(2.0) / f64x8::splat(3.0) * t38 * t550;
            let t673 = ((t82).select(f64x8::splat(0.0), t672));
            let t674 = param_c1 * t673;
            let t678 = t416 * t302;
            let t679 = t678 * t174;
            let t682 = t178 * t673;
            let t684 = -f64x8::splat(2.0) * t303 * t179 - t674 * t86 - f64x8::splat(2.0) * t84 * t679 - t84 * t682;
            let t685 = t684 * t88;
            let t686 = t307 * t181;
            let t689 = ((t90).select(f64x8::splat(0.0), t672));
            let t691 = t183 * t309;
            let t693 = t91 * t689;
            let t697 = t93 * t689;
            let t701 = t95 * t689;
            let t705 = t97 * t689;
            let t709 = t99 * t689;
            let t715 = -f64x8::splat(0.667) * t689 - f64x8::splat(0.889111) * t691 - f64x8::splat(0.889111) * t693 - f64x8::splat(3.978519606294) * t311 * t183 - f64x8::splat(1.989259803147) * t697 + f64x8::splat(17.41556453388) * t313 * t183 + f64x8::splat(5.80518817796) * t701 - f64x8::splat(17.75996083194) * t315 * t183 - f64x8::splat(4.439990207985) * t705 + f64x8::splat(7.03586824437) * t317 * t183 + f64x8::splat(1.407173648874) * t709 - f64x8::splat(0.973805419524) * t319 * t183 - f64x8::splat(0.162300903254) * t101 * t689;
            let t716 = t324 * t110;
            let t717 = t716 * t201;
            let t720 = ((t90).select(t672, f64x8::splat(0.0)));
            let t726 = ((t81).select(t686 * t88 + t685, (t89).select(t715, -t198 * t200 * t720 * t110 - f64x8::splat(2.0) * t605 * t717 - t614 * t717)));
            let t731 = t18 * t20;
            let t732 = t7 * t731;
            let t733 = t328 * t141;
            let t734 = t208 * t131;
            let t735 = t733 * t734;
            let t740 = t219 * t220 * t328 * t113;
            let t742 = t223 * t24 * t118;
            let t743 = t742 * t226;
            let t747 = ((t3).select(f64x8::splat(0.0), -t19 * t665 * t330 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t726 * t330 + f64x8::splat(3.0) / f64x8::splat(8.0) * t732 * t735 - f64x8::splat(1.6891736332904388) * t740 * t743));
            let tv2rhotau0 = f64x8::splat(2.0) * v_rho * t747 + f64x8::splat(2.0) * t334;
            acc_v2rhotau = tv2rhotau0;
            let t750 = t244 * t244;
            let t753 = t22 * v_sigma;
            let t756 = t147 * t35;
            let t758 = f64x8::splat(1.0) / t20 / t756;
            let t759 = t758 * t371;
            let t763 = f64x8::splat(1.740321555915593e-05) * t753 * t237 - f64x8::splat(4.0285221201749835e-08) * t146 * t759 * t375;
            let t765 = t38 * t549;
            let t766 = t397 * t32;
            let t767 = t766 * t37;
            let t770 = t396 * t767 / f64x8::splat(16.0) + t765 / f64x8::splat(16.0);
            let t771 = ((t82).select(f64x8::splat(0.0), t770));
            let t772 = param_c1 * t771;
            let t774 = t252 * t252;
            let t775 = param_c1 * t774;
            let t778 = t416 * t774;
            let t781 = t178 * t771;
            let t783 = -f64x8::splat(2.0) * t775 * t178 - t772 * t86 - f64x8::splat(2.0) * t84 * t778 - t84 * t781;
            let t785 = t257 * t257;
            let t786 = t785 * t88;
            let t788 = ((t90).select(f64x8::splat(0.0), t770));
            let t790 = t259 * t259;
            let t792 = t91 * t788;
            let t794 = t91 * t790;
            let t796 = t93 * t788;
            let t798 = t93 * t790;
            let t800 = t95 * t788;
            let t802 = t95 * t790;
            let t804 = t97 * t788;
            let t806 = t97 * t790;
            let t808 = t99 * t788;
            let t814 = -f64x8::splat(0.667) * t788 - f64x8::splat(0.889111) * t790 - f64x8::splat(0.889111) * t792 - f64x8::splat(3.978519606294) * t794 - f64x8::splat(1.989259803147) * t796 + f64x8::splat(17.41556453388) * t798 + f64x8::splat(5.80518817796) * t800 - f64x8::splat(17.75996083194) * t802 - f64x8::splat(4.439990207985) * t804 + f64x8::splat(7.03586824437) * t806 + f64x8::splat(1.407173648874) * t808 - f64x8::splat(0.973805419524) * t99 * t790 - f64x8::splat(0.162300903254) * t101 * t788;
            let t815 = t274 * t274;
            let t820 = ((t90).select(t770, f64x8::splat(0.0)));
            let t828 = ((t81).select(t783 * t88 + t786, (t89).select(t814, -t198 * t200 * t820 * t110 - f64x8::splat(2.0) * t198 * t455 * t815 * t110 - t466 * t468 * t815 * t110)));
            let t832 = t352 * t750;
            let t835 = t143 * t763;
            let t837 = t828 * t113 + t144 * t763 + f64x8::splat(2.0) * t207 * t832 - t207 * t835 - f64x8::splat(2.0) * t619 * t280 - f64x8::splat(2.0) * t353 * t750;
            let t844 = t219 * t287 * t282 * t223;
            let t848 = f64x8::splat(1.0) / t20 / t33;
            let t849 = t848 * t115;
            let t851 = t219 * t849 * t500;
            let t852 = f64x8::splat(1.0) / v_sigma;
            let t854 = t852 * t53 * t130;
            let t855 = t496 * t854;
            let t859 = t656 * t32 * t130;
            let t860 = t119 * t859;
            let t864 = t507 * t67 * t115;
            let t866 = f64x8::splat(1.0) / t120 / t30;
            let t868 = t866 * t23 * t512;
            let t872 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t837 * t131 + f64x8::splat(1.266880224967829) * t844 * t294 - f64x8::splat(2.8504805061776155) * t851 * t855 - f64x8::splat(0.31672005624195726) * t290 * t860 + f64x8::splat(3.8666622071141026) * t864 * t868));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t872;
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t876 = t32 * t494 * t549 / f64x8::splat(4.0);
            let t877 = ((t82).select(f64x8::splat(0.0), -t876));
            let t878 = param_c1 * t877;
            let t882 = t678 * t252;
            let t885 = t178 * t877;
            let t887 = -f64x8::splat(2.0) * t303 * t255 - f64x8::splat(2.0) * t84 * t882 - t84 * t885 - t878 * t86;
            let t888 = t887 * t88;
            let t889 = t307 * t257;
            let t892 = ((t90).select(f64x8::splat(0.0), -t876));
            let t894 = t259 * t309;
            let t896 = t91 * t892;
            let t900 = t93 * t892;
            let t904 = t95 * t892;
            let t908 = t97 * t892;
            let t912 = t99 * t892;
            let t918 = -f64x8::splat(0.667) * t892 - f64x8::splat(0.889111) * t894 - f64x8::splat(0.889111) * t896 - f64x8::splat(3.978519606294) * t311 * t259 - f64x8::splat(1.989259803147) * t900 + f64x8::splat(17.41556453388) * t313 * t259 + f64x8::splat(5.80518817796) * t904 - f64x8::splat(17.75996083194) * t315 * t259 - f64x8::splat(4.439990207985) * t908 + f64x8::splat(7.03586824437) * t317 * t259 + f64x8::splat(1.407173648874) * t912 - f64x8::splat(0.973805419524) * t319 * t259 - f64x8::splat(0.162300903254) * t101 * t892;
            let t919 = t716 * t274;
            let t922 = ((t90).select(-t876, f64x8::splat(0.0)));
            let t928 = ((t81).select(t889 * t88 + t888, (t89).select(t918, -t198 * t200 * t922 * t110 - f64x8::splat(2.0) * t605 * t919 - t614 * t919)));
            let t933 = t280 * t131;
            let t934 = t733 * t933;
            let t939 = t219 * t287 * t328 * t113;
            let t940 = t742 * t293;
            let t944 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t928 * t330 + f64x8::splat(3.0) / f64x8::splat(8.0) * t732 * t934 + f64x8::splat(0.6334401124839145) * t939 * t940));
            let tv2sigmatau0 = f64x8::splat(2.0) * v_rho * t944;
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t946 = ((t82).select(f64x8::splat(0.0), f64x8::splat(0.0)));
            let t947 = param_c1 * t946;
            let t948 = t947 * t86;
            let t949 = t302 * t302;
            let t950 = param_c1 * t949;
            let t953 = t416 * t949;
            let t957 = t84 * t178 * t946;
            let t958 = -f64x8::splat(2.0) * t950 * t178 - f64x8::splat(2.0) * t84 * t953 - t948 - t957;
            let t960 = t307 * t307;
            let t961 = t960 * t88;
            let t963 = ((t90).select(f64x8::splat(0.0), f64x8::splat(0.0)));
            let t964 = f64x8::splat(0.667) * t963;
            let t965 = t309 * t309;
            let t967 = t91 * t963;
            let t968 = f64x8::splat(0.889111) * t967;
            let t969 = t91 * t965;
            let t971 = t93 * t963;
            let t972 = f64x8::splat(1.989259803147) * t971;
            let t973 = t93 * t965;
            let t975 = t95 * t963;
            let t976 = f64x8::splat(5.80518817796) * t975;
            let t977 = t95 * t965;
            let t979 = t97 * t963;
            let t980 = f64x8::splat(4.439990207985) * t979;
            let t981 = t97 * t965;
            let t983 = t99 * t963;
            let t984 = f64x8::splat(1.407173648874) * t983;
            let t988 = f64x8::splat(0.162300903254) * t101 * t963;
            let t989 = -t964 - f64x8::splat(0.889111) * t965 - t968 - f64x8::splat(3.978519606294) * t969 - t972 + f64x8::splat(17.41556453388) * t973 + t976 - f64x8::splat(17.75996083194) * t977 - t980 + f64x8::splat(7.03586824437) * t981 + t984 - f64x8::splat(0.973805419524) * t99 * t965 - t988;
            let t990 = t324 * t324;
            let t997 = t198 * t200 * t963 * t110;
            let t1002 = ((t81).select(t958 * t88 + t961, (t89).select(t989, -f64x8::splat(2.0) * t198 * t455 * t990 * t110 - t466 * t468 * t990 * t110 - t997)));
            let t1007 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t1002 * t330));
            let tv2tau20 = f64x8::splat(2.0) * v_rho * t1007;
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
