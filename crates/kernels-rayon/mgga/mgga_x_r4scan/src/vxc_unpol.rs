//! MGGA_X_R4SCAN vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_r4scan.c`
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
pub fn mgga_x_r4scan_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_c1: f64,
    param_c2: f64,
    param_d: f64,
    param_da4: f64,
    param_dp2: f64,
    param_dp4: f64,
    param_eta: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c1 = f64x8::splat(param_c1);
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_da4 = f64x8::splat(param_da4);
    let param_dp2 = f64x8::splat(param_dp2);
    let param_dp4 = f64x8::splat(param_dp4);
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
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = t4 / t5 * t18;
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
            let t31 = t29 * t30;
            let t32 = f64x8::splat(M_CBRT2);
            let t33 = v_rho * v_rho;
            let t34 = t33 * t33;
            let t35 = t34 * v_rho;
            let t37 = f64x8::splat(1.0) / t20 / t35;
            let t38 = t32 * t37;
            let t39 = param_dp2 * param_dp2;
            let t40 = t39 * t39;
            let t41 = f64x8::splat(1.0) / t40;
            let t45 = (simd::exp(-t31 * t38 * t41 / f64x8::splat(288.0)));
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
            let t74 = param_eta * v_sigma;
            let t75 = t53 * t57;
            let t78 = f64x8::splat(3.0) / f64x8::splat(10.0) * t24 * t50 + t74 * t75 / f64x8::splat(8.0);
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
            let t116 = t22 * t23;
            let t117 = t116 * t51;
            let t120 = f64x8::splat(1.0) - t80;
            let t121 = t120 * t120;
            let t125 = (f64x8::splat(0.040570770199022686) - f64x8::splat(0.3023546802608101) * param_eta) * t23;
            let t126 = t125 * t51;
            let t133 = ((f64x8::splat(3.0) / f64x8::splat(4.0) * param_eta + f64x8::splat(2.0) / f64x8::splat(3.0)) * (f64x8::splat(3.0) / f64x8::splat(4.0) * param_eta + f64x8::splat(2.0) / f64x8::splat(3.0)));
            let t138 = ((f64x8::splat(0.0029070010613279013) - f64x8::splat(0.27123702538979) * param_eta) * (f64x8::splat(0.0029070010613279013) - f64x8::splat(0.27123702538979) * param_eta));
            let t142 = (f64x8::splat(146.0) / f64x8::splat(2025.0) * t133 - f64x8::splat(73.0) / f64x8::splat(540.0) * param_eta - f64x8::splat(146.0) / f64x8::splat(1215.0) + t138 / param_k1) * t24;
            let t143 = t142 * t28;
            let t144 = t30 * t32;
            let t145 = t144 * t37;
            let t148 = -f64x8::splat(0.162742215233874) + f64x8::splat(0.162742215233874) * t80 + f64x8::splat(0.00678092563474475) * t117 * t58 - f64x8::splat(0.059353125082804) * t121 + t126 * t54 * t57 * t120 / f64x8::splat(24.0) + t143 * t145 / f64x8::splat(288.0);
            let t149 = t71 * t71;
            let t150 = t148 * t149;
            let t151 = t78 * t78;
            let t152 = f64x8::splat(1.0) / t151;
            let t153 = t149 * t149;
            let t154 = t151 * t151;
            let t155 = f64x8::splat(1.0) / t154;
            let t157 = t153 * t155 + f64x8::splat(1.0);
            let t158 = f64x8::splat(1.0) / t157;
            let t159 = t152 * t158;
            let t160 = param_da4 * param_da4;
            let t161 = f64x8::splat(1.0) / t160;
            let t163 = param_dp4 * param_dp4;
            let t164 = t163 * t163;
            let t165 = f64x8::splat(1.0) / t164;
            let t166 = t38 * t165;
            let t170 = (simd::exp(-t121 * t161 - t31 * t166 / f64x8::splat(288.0)));
            let t171 = t159 * t170;
            let t174 = t112 * t113 + f64x8::splat(2.0) * t150 * t171 + t65 + f64x8::splat(1.0);
            let t176 = ((f64x8::splat(3.0)).sqrt());
            let t177 = f64x8::splat(1.0) / t26;
            let t178 = t24 * t177;
            let t179 = ((v_sigma).sqrt());
            let t180 = t179 * t32;
            let t182 = f64x8::splat(1.0) / t20 / v_rho;
            let t184 = t178 * t180 * t182;
            let t185 = ((t184).sqrt());
            let t189 = (simd::exp(-f64x8::splat(9.8958) * t176 / t185));
            let t190 = f64x8::splat(1.0) - t189;
            let t194 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t174 * t190));
            let tzk0 = f64x8::splat(2.0) * t194;
            acc_zk = tzk0;
            let t195 = f64x8::splat(1.0) / t55;
            let t200 = param_k1 * param_k1;
            let t201 = t61 * t61;
            let t202 = f64x8::splat(1.0) / t201;
            let t203 = t200 * t202;
            let t204 = t30 * v_sigma;
            let t205 = t22 * t204;
            let t206 = t34 * t34;
            let t207 = t206 * v_rho;
            let t208 = f64x8::splat(1.0) / t207;
            let t210 = t208 * t41 * t45;
            let t213 = t33 * v_rho;
            let t215 = f64x8::splat(1.0) / t55 / t213;
            let t216 = t54 * t215;
            let t219 = -f64x8::splat(1.5469524941471938e-05) * t205 * t210 - t52 * t216 / f64x8::splat(9.0);
            let t224 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t66 * t57 + t216 / f64x8::splat(3.0);
            let t225 = t224 * t79;
            let t226 = t71 * t152;
            let t227 = t226 * param_eta;
            let t228 = t227 * t216;
            let t230 = t225 + t228 / f64x8::splat(3.0);
            let t231 = ((t82).select(f64x8::splat(0.0), t230));
            let t234 = t85 * t85;
            let t235 = f64x8::splat(1.0) / t234;
            let t236 = t235 * t231;
            let t238 = -param_c1 * t231 * t86 - t84 * t236;
            let t239 = t238 * t88;
            let t240 = ((t90).select(f64x8::splat(0.0), t230));
            let t242 = t91 * t240;
            let t244 = t93 * t240;
            let t246 = t95 * t240;
            let t248 = t97 * t240;
            let t250 = t99 * t240;
            let t255 = param_d * param_c2;
            let t256 = t107 * t107;
            let t257 = f64x8::splat(1.0) / t256;
            let t258 = ((t90).select(t230, f64x8::splat(0.0)));
            let t262 = ((t81).select(t239, (t89).select(-f64x8::splat(0.667) * t240 - f64x8::splat(0.889111) * t242 - f64x8::splat(1.989259803147) * t244 + f64x8::splat(5.80518817796) * t246 - f64x8::splat(4.439990207985) * t248 + f64x8::splat(1.407173648874) * t250 - f64x8::splat(0.162300903254) * t101 * t240, -t255 * t257 * t258 * t110)));
            let t264 = t112 * t200;
            let t265 = t202 * t219;
            let t271 = -t230;
            let t282 = t34 * t33;
            let t284 = f64x8::splat(1.0) / t20 / t282;
            let t288 = f64x8::splat(0.162742215233874) * t225 + f64x8::splat(0.054247405077958) * t228 - f64x8::splat(0.018082468359319332) * t117 * t216 - f64x8::splat(0.118706250165608) * t120 * t271 - t126 * t54 * t215 * t120 / f64x8::splat(9.0) + t126 * t54 * t57 * t271 / f64x8::splat(24.0) - t143 * t144 * t284 / f64x8::splat(54.0);
            let t289 = t288 * t149;
            let t292 = t148 * t71;
            let t293 = t292 * t152;
            let t294 = t158 * t170;
            let t295 = t294 * t224;
            let t298 = t151 * t78;
            let t299 = f64x8::splat(1.0) / t298;
            let t300 = t299 * t158;
            let t301 = t150 * t300;
            let t302 = t170 * param_eta;
            let t303 = t302 * t216;
            let t306 = t150 * t152;
            let t307 = t157 * t157;
            let t308 = f64x8::splat(1.0) / t307;
            let t309 = t308 * t170;
            let t310 = t149 * t71;
            let t311 = t310 * t155;
            let t315 = f64x8::splat(1.0) / t154 / t78;
            let t316 = t153 * t315;
            let t317 = t316 * param_eta;
            let t320 = f64x8::splat(4.0) * t311 * t224 + f64x8::splat(4.0) / f64x8::splat(3.0) * t317 * t216;
            let t321 = t309 * t320;
            let t324 = t120 * t161;
            let t327 = t32 * t284;
            let t328 = t327 * t165;
            let t331 = -f64x8::splat(2.0) * t324 * t271 + t31 * t328 / f64x8::splat(54.0);
            let t332 = t158 * t331;
            let t333 = t332 * t170;
            let t336 = t203 * t219 + t262 * t113 - t264 * t265 + f64x8::splat(2.0) * t289 * t171 + f64x8::splat(4.0) * t293 * t295 + f64x8::splat(4.0) / f64x8::splat(3.0) * t301 * t303 - f64x8::splat(2.0) * t306 * t321 + f64x8::splat(2.0) * t306 * t333;
            let t341 = (simd::pow(f64x8::splat(3.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t342 = t341 * t341;
            let t343 = t342 * t342;
            let t345 = t343 * t341 * t18;
            let t346 = f64x8::splat(1.0) / t33;
            let t347 = t346 * t174;
            let t349 = f64x8::splat(1.0) / t185 / t184;
            let t351 = t345 * t347 * t349;
            let t353 = t178 * t180 * t189;
            let t357 = ((t3).select(f64x8::splat(0.0), -t19 * t195 * t174 * t190 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t336 * t190 - f64x8::splat(1.6891736332904388) * t351 * t353));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t357 + f64x8::splat(2.0) * t194;
            acc_vrho = tvrho0;
            let t360 = t22 * t30;
            let t361 = f64x8::splat(1.0) / t206;
            let t363 = t361 * t41 * t45;
            let t366 = t51 * t53;
            let t367 = t366 * t57;
            let t370 = f64x8::splat(5.801071853051976e-06) * t360 * t363 + t49 * t367 / f64x8::splat(24.0);
            let t372 = t75 * t79;
            let t373 = param_eta * t53;
            let t374 = t373 * t57;
            let t375 = t226 * t374;
            let t377 = -t372 / f64x8::splat(8.0) - t375 / f64x8::splat(8.0);
            let t378 = ((t82).select(f64x8::splat(0.0), t377));
            let t379 = param_c1 * t378;
            let t381 = t235 * t378;
            let t383 = -t379 * t86 - t84 * t381;
            let t384 = t383 * t88;
            let t385 = ((t90).select(f64x8::splat(0.0), t377));
            let t387 = t91 * t385;
            let t389 = t93 * t385;
            let t391 = t95 * t385;
            let t393 = t97 * t385;
            let t395 = t99 * t385;
            let t400 = ((t90).select(t377, f64x8::splat(0.0)));
            let t404 = ((t81).select(t384, (t89).select(-f64x8::splat(0.667) * t385 - f64x8::splat(0.889111) * t387 - f64x8::splat(1.989259803147) * t389 + f64x8::splat(5.80518817796) * t391 - f64x8::splat(4.439990207985) * t393 + f64x8::splat(1.407173648874) * t395 - f64x8::splat(0.162300903254) * t101 * t385, -t255 * t257 * t400 * t110)));
            let t406 = t202 * t370;
            let t412 = -t377;
            let t422 = v_sigma * t32;
            let t426 = -f64x8::splat(0.02034277690423425) * t372 - f64x8::splat(0.02034277690423425) * t375 + f64x8::splat(0.00678092563474475) * t116 * t367 - f64x8::splat(0.118706250165608) * t120 * t412 + t126 * t75 * t120 / f64x8::splat(24.0) + t126 * t54 * t57 * t412 / f64x8::splat(24.0) + t143 * t422 * t37 / f64x8::splat(144.0);
            let t427 = t426 * t149;
            let t430 = t294 * t75;
            let t431 = t293 * t430;
            let t433 = t302 * t75;
            let t439 = -t311 * t75 / f64x8::splat(2.0) - t316 * t374 / f64x8::splat(2.0);
            let t440 = t309 * t439;
            let t445 = t29 * v_sigma;
            let t448 = -f64x8::splat(2.0) * t324 * t412 - t445 * t166 / f64x8::splat(144.0);
            let t449 = t158 * t448;
            let t450 = t449 * t170;
            let t453 = t203 * t370 + t404 * t113 - t264 * t406 + f64x8::splat(2.0) * t427 * t171 - t431 / f64x8::splat(2.0) - t301 * t433 / f64x8::splat(2.0) - f64x8::splat(2.0) * t306 * t440 + f64x8::splat(2.0) * t306 * t450;
            let t458 = f64x8::splat(1.0) / v_rho;
            let t459 = t458 * t174;
            let t461 = t345 * t459 * t349;
            let t462 = f64x8::splat(1.0) / t179;
            let t465 = t178 * t462 * t32 * t189;
            let t469 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t453 * t190 + f64x8::splat(0.6334401124839145) * t461 * t465));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t469;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t471 = t53 * t68;
            let t472 = t471 * t79;
            let t473 = ((t82).select(f64x8::splat(0.0), t472));
            let t474 = param_c1 * t473;
            let t476 = t235 * t473;
            let t478 = -t474 * t86 - t84 * t476;
            let t479 = t478 * t88;
            let t480 = ((t90).select(f64x8::splat(0.0), t472));
            let t482 = t91 * t480;
            let t484 = t93 * t480;
            let t486 = t95 * t480;
            let t488 = t97 * t480;
            let t490 = t99 * t480;
            let t495 = ((t90).select(t472, f64x8::splat(0.0)));
            let t499 = ((t81).select(t479, (t89).select(-f64x8::splat(0.667) * t480 - f64x8::splat(0.889111) * t482 - f64x8::splat(1.989259803147) * t484 + f64x8::splat(5.80518817796) * t486 - f64x8::splat(4.439990207985) * t488 + f64x8::splat(1.407173648874) * t490 - f64x8::splat(0.162300903254) * t101 * t480, -t255 * t257 * t495 * t110)));
            let t502 = t120 * t53;
            let t503 = t68 * t79;
            let t507 = f64x8::splat(1.0) / t20 / t34;
            let t512 = f64x8::splat(0.162742215233874) * t472 + f64x8::splat(0.118706250165608) * t502 * t503 - t126 * t422 * t507 * t79 / f64x8::splat(12.0);
            let t513 = t512 * t149;
            let t516 = t294 * t471;
            let t519 = t153 * t71;
            let t520 = t148 * t519;
            let t521 = t154 * t151;
            let t522 = f64x8::splat(1.0) / t521;
            let t523 = t520 * t522;
            let t524 = t309 * t471;
            let t527 = t471 * t170;
            let t528 = t324 * t527;
            let t531 = t499 * t113 + f64x8::splat(2.0) * t513 * t171 + f64x8::splat(4.0) * t293 * t516 + f64x8::splat(4.0) * t301 * t528 - f64x8::splat(8.0) * t523 * t524;
            let t536 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t531 * t190));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t536;
            acc_vtau = tvtau0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
