//! GGA_C_PBE_VWN fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pbe_vwn.c`
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
pub fn gga_c_pbe_vwn_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_gamma: f64,
    param_BB: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_gamma = f64x8::splat(param_gamma);
    let param_BB = f64x8::splat(param_BB);
    let param_beta = f64x8::splat(param_beta);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = t6 * t8;
            let t10 = t4 * t9;
            let t11 = t10 / f64x8::splat(4.0);
            let t12 = ((t10).sqrt());
            let t14 = t11 + f64x8::splat(1.86372) * t12 + f64x8::splat(12.9352);
            let t15 = f64x8::splat(1.0) / t14;
            let t19 = (simd::ln(t4 * t9 * t15 / f64x8::splat(4.0)));
            let t20 = f64x8::splat(0.0310907) * t19;
            let t21 = t12 + f64x8::splat(3.72744);
            let t24 = (simd::atan(f64x8::splat(6.15199081975908) / t21));
            let t25 = f64x8::splat(0.038783294878113016) * t24;
            let t26 = t12 / f64x8::splat(2.0);
            let t27 = t26 + f64x8::splat(0.10498);
            let t28 = t27 * t27;
            let t30 = (simd::ln(t28 * t15));
            let t31 = f64x8::splat(0.0009690227711544374) * t30;
            let t32 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t33 = f64x8::splat(1.0) / t32;
            let t35 = t11 + f64x8::splat(0.565535) * t12 + f64x8::splat(13.0045);
            let t36 = f64x8::splat(1.0) / t35;
            let t40 = (simd::ln(t4 * t9 * t36 / f64x8::splat(4.0)));
            let t41 = t12 + f64x8::splat(1.13107);
            let t44 = (simd::atan(f64x8::splat(7.123108917818118) / t41));
            let t46 = t26 + f64x8::splat(0.0047584);
            let t47 = t46 * t46;
            let t49 = (simd::ln(t47 * t36));
            let t53 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t54 = (simd::cbrt(zeta_threshold));
            let t56 = ((t53).select(t54 * zeta_threshold, f64x8::splat(1.0)));
            let t59 = f64x8::splat(M_CBRT2);
            let t60 = t59 - f64x8::splat(1.0);
            let t65 = f64x8::splat(9.0) * t56 - f64x8::splat(9.0);
            let t67 = t33 * (t40 + f64x8::splat(0.31770800474394145) * t44 + f64x8::splat(0.00041403379428206277) * t49) * t65 / f64x8::splat(24.0);
            let t68 = t54 * t54;
            let t69 = ((t53).select(t68, f64x8::splat(1.0)));
            let t70 = t69 * t69;
            let t71 = t70 * t69;
            let t72 = param_gamma * t71;
            let t73 = v_rho * v_rho;
            let t75 = f64x8::splat(1.0) / t7 / t73;
            let t78 = f64x8::splat(1.0) / t70;
            let t79 = t1 * t1;
            let t81 = f64x8::splat(1.0) / t3;
            let t82 = t81 * t5;
            let t83 = t78 * t79 * t82;
            let t86 = param_BB * param_beta;
            let t87 = f64x8::splat(1.0) / param_gamma;
            let t90 = f64x8::splat(1.0) / t71;
            let t92 = (simd::exp(-(t20 + t25 + t31 - t67) * t87 * t90));
            let t93 = t92 - f64x8::splat(1.0);
            let t94 = f64x8::splat(1.0) / t93;
            let t95 = t87 * t94;
            let t96 = v_sigma * v_sigma;
            let t98 = t86 * t95 * t96;
            let t99 = t73 * t73;
            let t100 = t7 * t7;
            let t102 = f64x8::splat(1.0) / t100 / t99;
            let t103 = t59 * t59;
            let t104 = t102 * t103;
            let t105 = t70 * t70;
            let t106 = f64x8::splat(1.0) / t105;
            let t107 = t104 * t106;
            let t108 = t3 * t3;
            let t109 = f64x8::splat(1.0) / t108;
            let t110 = t1 * t109;
            let t111 = t110 * t6;
            let t112 = t107 * t111;
            let t115 = v_sigma * t75 * t59 * t83 / f64x8::splat(96.0) + t98 * t112 / f64x8::splat(3072.0);
            let t116 = param_beta * t115;
            let t117 = param_beta * t87;
            let t120 = t117 * t94 * t115 + f64x8::splat(1.0);
            let t121 = f64x8::splat(1.0) / t120;
            let t122 = t87 * t121;
            let t124 = t116 * t122 + f64x8::splat(1.0);
            let t125 = (simd::ln(t124));
            let t126 = t72 * t125;
            let tzk0 = t20 + t25 + t31 - t67 + t126;
            acc_zk = tzk0;
            let t128 = f64x8::splat(1.0) / t7 / v_rho;
            let t129 = t6 * t128;
            let t133 = t4 * t6;
            let t134 = t14 * t14;
            let t135 = f64x8::splat(1.0) / t134;
            let t136 = t8 * t135;
            let t137 = t4 * t129;
            let t138 = t137 / f64x8::splat(12.0);
            let t139 = f64x8::splat(1.0) / t12;
            let t140 = t139 * t1;
            let t141 = t3 * t6;
            let t143 = t140 * t141 * t128;
            let t145 = -t138 - f64x8::splat(0.31062) * t143;
            let t151 = (-t4 * t129 * t15 / f64x8::splat(12.0) - t133 * t136 * t145 / f64x8::splat(4.0)) * t79 * t81;
            let t152 = t5 * t7;
            let t153 = t152 * t14;
            let t154 = t151 * t153;
            let t155 = f64x8::splat(0.010363566666666667) * t154;
            let t156 = t21 * t21;
            let t157 = f64x8::splat(1.0) / t156;
            let t159 = t157 * t139 * t1;
            let t161 = f64x8::splat(37.8469910464) * t157 + f64x8::splat(1.0);
            let t162 = f64x8::splat(1.0) / t161;
            let t165 = t159 * t141 * t128 * t162;
            let t166 = f64x8::splat(0.03976574567502677) * t165;
            let t167 = t27 * t15;
            let t168 = t167 * t139;
            let t171 = t28 * t135;
            let t173 = -t168 * t137 / f64x8::splat(6.0) - t171 * t145;
            let t174 = f64x8::splat(1.0) / t28;
            let t175 = t173 * t174;
            let t176 = t175 * t14;
            let t177 = f64x8::splat(0.0009690227711544374) * t176;
            let t181 = t35 * t35;
            let t182 = f64x8::splat(1.0) / t181;
            let t183 = t8 * t182;
            let t185 = -t138 - f64x8::splat(0.09425583333333333) * t143;
            let t191 = (-t4 * t129 * t36 / f64x8::splat(12.0) - t133 * t183 * t185 / f64x8::splat(4.0)) * t79 * t81;
            let t192 = t152 * t35;
            let t195 = t41 * t41;
            let t196 = f64x8::splat(1.0) / t195;
            let t198 = t196 * t139 * t1;
            let t200 = f64x8::splat(50.7386806551) * t196 + f64x8::splat(1.0);
            let t201 = f64x8::splat(1.0) / t200;
            let t206 = t46 * t36;
            let t207 = t206 * t139;
            let t210 = t47 * t182;
            let t212 = -t207 * t137 / f64x8::splat(6.0) - t210 * t185;
            let t213 = f64x8::splat(1.0) / t47;
            let t214 = t212 * t213;
            let t219 = t33 * (t191 * t192 / f64x8::splat(3.0) + f64x8::splat(0.37717812030896175) * t198 * t141 * t128 * t201 + f64x8::splat(0.00041403379428206277) * t214 * t35) * t65;
            let t220 = t219 / f64x8::splat(24.0);
            let t221 = t73 * v_rho;
            let t223 = f64x8::splat(1.0) / t7 / t221;
            let t228 = param_gamma * param_gamma;
            let t229 = f64x8::splat(1.0) / t228;
            let t230 = t86 * t229;
            let t231 = t93 * t93;
            let t232 = f64x8::splat(1.0) / t231;
            let t233 = t232 * t96;
            let t234 = t233 * t102;
            let t235 = t230 * t234;
            let t237 = f64x8::splat(1.0) / t105 / t71;
            let t238 = t103 * t237;
            let t239 = t238 * t1;
            let t240 = t109 * t6;
            let t241 = t155 + t166 + t177 - t220;
            let t242 = t241 * t92;
            let t243 = t240 * t242;
            let t244 = t239 * t243;
            let t247 = t99 * v_rho;
            let t249 = f64x8::splat(1.0) / t100 / t247;
            let t250 = t249 * t103;
            let t251 = t250 * t106;
            let t252 = t251 * t111;
            let t255 = -f64x8::splat(7.0) / f64x8::splat(288.0) * v_sigma * t223 * t59 * t83 + t235 * t244 / f64x8::splat(3072.0) - f64x8::splat(7.0) / f64x8::splat(4608.0) * t98 * t252;
            let t256 = param_beta * t255;
            let t258 = t120 * t120;
            let t259 = f64x8::splat(1.0) / t258;
            let t260 = t87 * t259;
            let t262 = param_beta * t229 * t232;
            let t264 = t90 * t92;
            let t269 = t262 * t115 * t241 * t264 + t117 * t94 * t255;
            let t270 = t260 * t269;
            let t272 = -t116 * t270 + t256 * t122;
            let t273 = f64x8::splat(1.0) / t124;
            let t275 = t72 * t272 * t273;
            let tvrho0 = t20 + t25 + t31 - t67 + t126 + v_rho * (t155 + t166 + t177 - t220 + t275);
            acc_vrho = tvrho0;
            let t278 = v_rho * param_gamma;
            let t282 = t79 * t81 * t5;
            let t286 = t86 * t95 * v_sigma;
            let t289 = t75 * t59 * t78 * t282 / f64x8::splat(96.0) + t286 * t112 / f64x8::splat(1536.0);
            let t290 = param_beta * t289;
            let t292 = param_beta * param_beta;
            let t293 = t292 * t115;
            let t294 = t293 * t229;
            let t295 = t259 * t94;
            let t296 = t295 * t289;
            let t298 = t290 * t122 - t294 * t296;
            let tvsigma0 = t278 * t71 * t298 * t273;
            acc_vsigma = tvsigma0;
            let t306 = t6 * t75;
            let t308 = t4 * t306 * t15;
            let t310 = t128 * t135;
            let t315 = f64x8::splat(1.0) / t134 / t14;
            let t316 = t8 * t315;
            let t317 = t145 * t145;
            let t321 = t4 * t306;
            let t322 = t321 / f64x8::splat(9.0);
            let t324 = f64x8::splat(1.0) / t12 / t10;
            let t325 = t324 * t79;
            let t326 = t108 * t5;
            let t328 = f64x8::splat(1.0) / t100 / t73;
            let t330 = t325 * t326 * t328;
            let t333 = t140 * t141 * t75;
            let t335 = t322 - f64x8::splat(0.20708) * t330 + f64x8::splat(0.41416) * t333;
            let t341 = (t308 / f64x8::splat(9.0) + t133 * t310 * t145 / f64x8::splat(6.0) + t133 * t316 * t317 / f64x8::splat(2.0) - t133 * t136 * t335 / f64x8::splat(4.0)) * t79 * t81;
            let t342 = t341 * t153;
            let t343 = f64x8::splat(0.010363566666666667) * t342;
            let t345 = t5 / t100;
            let t346 = t345 * t14;
            let t347 = t151 * t346;
            let t348 = f64x8::splat(0.003454522222222222) * t347;
            let t349 = t152 * t145;
            let t350 = t151 * t349;
            let t351 = f64x8::splat(0.010363566666666667) * t350;
            let t352 = t156 * t21;
            let t353 = f64x8::splat(1.0) / t352;
            let t354 = t353 * t1;
            let t355 = t354 * t3;
            let t357 = t355 * t306 * t162;
            let t358 = f64x8::splat(0.013255248558342257) * t357;
            let t360 = t157 * t324 * t79;
            let t363 = t360 * t326 * t328 * t162;
            let t364 = f64x8::splat(0.026510497116684514) * t363;
            let t367 = t159 * t141 * t75 * t162;
            let t368 = f64x8::splat(0.05302099423336903) * t367;
            let t369 = t156 * t156;
            let t371 = f64x8::splat(1.0) / t369 / t21;
            let t372 = t371 * t1;
            let t373 = t372 * t3;
            let t374 = t161 * t161;
            let t375 = f64x8::splat(1.0) / t374;
            let t377 = t373 * t306 * t375;
            let t378 = f64x8::splat(0.5016712735053859) * t377;
            let t380 = t27 * t135;
            let t381 = t380 * t140;
            let t382 = t128 * t145;
            let t386 = t167 * t324;
            let t387 = t79 * t108;
            let t388 = t5 * t328;
            let t389 = t387 * t388;
            let t394 = t28 * t315;
            let t398 = t308 / f64x8::splat(72.0) + t381 * t141 * t382 / f64x8::splat(3.0) - t386 * t389 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t168 * t321 + f64x8::splat(2.0) * t394 * t317 - t171 * t335;
            let t399 = t398 * t174;
            let t400 = t399 * t14;
            let t401 = f64x8::splat(0.0009690227711544374) * t400;
            let t403 = f64x8::splat(1.0) / t28 / t27;
            let t404 = t173 * t403;
            let t405 = t14 * t139;
            let t406 = t404 * t405;
            let t407 = t406 * t137;
            let t408 = f64x8::splat(0.00016150379519240624) * t407;
            let t409 = t175 * t145;
            let t410 = f64x8::splat(0.0009690227711544374) * t409;
            let t412 = t4 * t306 * t36;
            let t414 = t128 * t182;
            let t419 = f64x8::splat(1.0) / t181 / t35;
            let t420 = t8 * t419;
            let t421 = t185 * t185;
            let t427 = t322 - f64x8::splat(0.06283722222222222) * t330 + f64x8::splat(0.12567444444444445) * t333;
            let t433 = (t412 / f64x8::splat(9.0) + t133 * t414 * t185 / f64x8::splat(6.0) + t133 * t420 * t421 / f64x8::splat(2.0) - t133 * t183 * t427 / f64x8::splat(4.0)) * t79 * t81;
            let t436 = t345 * t35;
            let t439 = t152 * t185;
            let t442 = t195 * t41;
            let t443 = f64x8::splat(1.0) / t442;
            let t444 = t443 * t1;
            let t445 = t444 * t3;
            let t450 = t196 * t324 * t79;
            let t459 = t195 * t195;
            let t461 = f64x8::splat(1.0) / t459 / t41;
            let t462 = t461 * t1;
            let t463 = t462 * t3;
            let t464 = t200 * t200;
            let t465 = f64x8::splat(1.0) / t464;
            let t470 = t46 * t182;
            let t471 = t470 * t140;
            let t472 = t128 * t185;
            let t476 = t206 * t324;
            let t481 = t47 * t419;
            let t485 = t412 / f64x8::splat(72.0) + t471 * t141 * t472 / f64x8::splat(3.0) - t476 * t389 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t207 * t321 + f64x8::splat(2.0) * t481 * t421 - t210 * t427;
            let t486 = t485 * t213;
            let t490 = f64x8::splat(1.0) / t47 / t46;
            let t491 = t212 * t490;
            let t492 = t35 * t139;
            let t493 = t491 * t492;
            let t500 = t33 * (t433 * t192 / f64x8::splat(3.0) + t191 * t436 / f64x8::splat(9.0) + t191 * t439 / f64x8::splat(3.0) + f64x8::splat(0.12572604010298724) * t445 * t306 * t201 + f64x8::splat(0.2514520802059745) * t450 * t326 * t328 * t201 - f64x8::splat(0.502904160411949) * t198 * t141 * t75 * t201 - f64x8::splat(6.379173398815766) * t463 * t306 * t465 + f64x8::splat(0.00041403379428206277) * t486 * t35 + f64x8::splat(6.900563238034379e-05) * t493 * t137 + f64x8::splat(0.00041403379428206277) * t214 * t185) * t65;
            let t501 = t500 / f64x8::splat(24.0);
            let t503 = f64x8::splat(1.0) / t7 / t99;
            let t509 = f64x8::splat(1.0) / t228 / param_gamma;
            let t510 = t86 * t509;
            let t512 = f64x8::splat(1.0) / t231 / t93;
            let t513 = t512 * t96;
            let t514 = t513 * t102;
            let t515 = t510 * t514;
            let t516 = t105 * t105;
            let t518 = f64x8::splat(1.0) / t516 / t70;
            let t520 = t103 * t518 * t1;
            let t521 = t241 * t241;
            let t522 = t92 * t92;
            let t523 = t521 * t522;
            let t525 = t520 * t240 * t523;
            let t528 = t233 * t249;
            let t529 = t230 * t528;
            let t532 = t343 + t348 + t351 + t358 + t364 - t368 - t378 + t401 + t408 + t410 - t501;
            let t533 = t532 * t92;
            let t535 = t239 * t240 * t533;
            let t538 = t510 * t234;
            let t539 = t521 * t92;
            let t541 = t520 * t240 * t539;
            let t544 = t99 * t73;
            let t546 = f64x8::splat(1.0) / t100 / t544;
            let t549 = t546 * t103 * t106 * t111;
            let t552 = f64x8::splat(35.0) / f64x8::splat(432.0) * v_sigma * t503 * t59 * t83 + t515 * t525 / f64x8::splat(1536.0) - f64x8::splat(7.0) / f64x8::splat(2304.0) * t529 * t244 + t235 * t535 / f64x8::splat(3072.0) - t538 * t541 / f64x8::splat(3072.0) + f64x8::splat(119.0) / f64x8::splat(13824.0) * t98 * t549;
            let t553 = param_beta * t552;
            let t558 = f64x8::splat(1.0) / t258 / t120;
            let t559 = t87 * t558;
            let t560 = t269 * t269;
            let t561 = t559 * t560;
            let t564 = param_beta * t509;
            let t565 = t564 * t512;
            let t566 = t115 * t521;
            let t568 = f64x8::splat(1.0) / t105 / t70;
            let t569 = t568 * t522;
            let t580 = t564 * t232;
            let t581 = t568 * t92;
            let t586 = t262 * t115 * t532 * t264 + f64x8::splat(2.0) * t262 * t255 * t241 * t264 + t117 * t94 * t552 + f64x8::splat(2.0) * t565 * t566 * t569 - t580 * t566 * t581;
            let t587 = t260 * t586;
            let t589 = f64x8::splat(2.0) * t116 * t561 - t116 * t587 + t553 * t122 - f64x8::splat(2.0) * t256 * t270;
            let t591 = t72 * t589 * t273;
            let t592 = t272 * t272;
            let t593 = t124 * t124;
            let t594 = f64x8::splat(1.0) / t593;
            let t596 = t72 * t592 * t594;
            let t597 = t343 + t348 + t351 + t358 + t364 - t368 - t378 + t401 + t408 + t410 - t501 + t591 - t596;
            let tv2rho20 = f64x8::splat(0.020727133333333335) * t154 + f64x8::splat(0.07953149135005354) * t165 + f64x8::splat(0.001938045542308875) * t176 - t219 / f64x8::splat(12.0) + f64x8::splat(2.0) * t275 + v_rho * t597;
            acc_v2rho2 = tv2rho20;
            let t605 = t232 * v_sigma;
            let t606 = t605 * t102;
            let t607 = t230 * t606;
            let t612 = -f64x8::splat(7.0) / f64x8::splat(288.0) * t223 * t59 * t78 * t282 + t607 * t244 / f64x8::splat(1536.0) - f64x8::splat(7.0) / f64x8::splat(2304.0) * t286 * t252;
            let t613 = param_beta * t612;
            let t616 = t292 * t255;
            let t617 = t616 * t229;
            let t619 = t558 * t94;
            let t620 = t289 * t269;
            let t621 = t619 * t620;
            let t624 = t509 * t259;
            let t625 = t293 * t624;
            let t626 = t232 * t289;
            let t628 = t241 * t90 * t92;
            let t629 = t626 * t628;
            let t631 = t295 * t612;
            let t633 = t613 * t122 - t290 * t270 + f64x8::splat(2.0) * t294 * t621 - t294 * t631 - t617 * t296 - t625 * t629;
            let t637 = t278 * t71;
            let t638 = t298 * t594;
            let t639 = t638 * t272;
            let tv2rhosigma0 = t278 * t71 * t633 * t273 + t72 * t298 * t273 - t637 * t639;
            acc_v2rhosigma = tv2rhosigma0;
            let t641 = t292 * param_BB;
            let t642 = t229 * t94;
            let t648 = t103 * t106 * t1 * t240 * t121;
            let t651 = t289 * t289;
            let t652 = t292 * t651;
            let t654 = t229 * t259 * t94;
            let t657 = t292 * param_beta;
            let t658 = t657 * t115;
            let t659 = t658 * t509;
            let t660 = t558 * t232;
            let t661 = t660 * t651;
            let t665 = t259 * t232 * param_BB;
            let t666 = t659 * t665;
            let t669 = t641 * t642 * t102 * t648 / f64x8::splat(1536.0) - f64x8::splat(2.0) * t652 * t654 + f64x8::splat(2.0) * t659 * t661 - t666 * t112 / f64x8::splat(1536.0);
            let t673 = t298 * t298;
            let tv2sigma20 = t278 * t71 * t669 * t273 - t278 * t71 * t673 * t594;
            acc_v2sigma2 = tv2sigma20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
