//! GGA_C_LYP lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lyp.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_lyp_lxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
    param_a: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t2 = 1.0 / t1;
        let t4 = param_d * t2 + 1.0;
        let t5 = 1.0 / t4;
        let t7 = rmath::exp(-param_c * t2);
        let t8 = param_b * t7;
        let t9 = rho[ip] * rho[ip];
        let t10 = t1 * t1;
        let t12 = 1.0 / t10 / t9;
        let t13 = sigma[ip] * t12;
        let t15 = param_d * t5 + param_c;
        let t16 = t15 * t2;
        let t18 = -1.0 / 72.0 - 7.0 / 72.0 * t16;
        let t20 = M_CBRT3;
        let t21 = t20 * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t26 = 1.0 <= zeta_threshold;
        let t27 = zeta_threshold * zeta_threshold;
        let t28 = pow_1_3(zeta_threshold);
        let t29 = t28 * t28;
        let t31 = piecewise3(t26, t29 * t27, 1.0);
        let t35 = 5.0 / 2.0 - t16 / 18.0;
        let t36 = t35 * sigma[ip];
        let t37 = t12 * t31;
        let t40 = t16 - 11.0;
        let t41 = t40 * sigma[ip];
        let t44 = piecewise3(t26, t29 * t27 * zeta_threshold, 1.0);
        let t45 = t12 * t44;
        let t48 = M_CBRT2;
        let t49 = t48 * t48;
        let t50 = sigma[ip] * t49;
        let t53 = piecewise3(t26, t27, 1.0);
        let t54 = t53 * sigma[ip];
        let t56 = t49 * t12 * t31;
        let t62 = -t13 * t18 - 3.0 / 10.0 * t21 * t24 * t31 + t36 * t37 / 8.0 + t41 * t45 / 144.0 - t48 * (4.0 / 3.0 * t50 * t37 - t54 * t56 / 2.0) / 8.0;
        let tzk0 = param_a * (t8 * t5 * t62 - t5);
        zk[ip] += tzk0;
        let t66 = rho[ip] * param_a;
        let t67 = t4 * t4;
        let t68 = 1.0 / t67;
        let t69 = t68 * param_d;
        let t71 = 1.0 / t1 / rho[ip];
        let t74 = param_b * param_c;
        let t75 = t74 * t71;
        let t76 = t7 * t5;
        let t77 = t76 * t62;
        let t80 = t8 * t68;
        let t81 = t62 * param_d;
        let t85 = t9 * rho[ip];
        let t87 = 1.0 / t10 / t85;
        let t88 = sigma[ip] * t87;
        let t91 = param_d * param_d;
        let t92 = t91 * t68;
        let t94 = 1.0 / t10 / rho[ip];
        let t97 = t15 * t71 - t92 * t94;
        let t98 = 7.0 / 216.0 * t97;
        let t100 = t97 / 54.0;
        let t101 = t100 * sigma[ip];
        let t104 = t87 * t31;
        let t108 = -t97 / 3.0;
        let t109 = t108 * sigma[ip];
        let t112 = t87 * t44;
        let t118 = t49 * t87 * t31;
        let t124 = 8.0 / 3.0 * t88 * t18 - t13 * t98 + t101 * t37 / 8.0 - t36 * t104 / 3.0 + t109 * t45 / 144.0 - t41 * t112 / 54.0 - t48 * (-32.0 / 9.0 * t50 * t104 + 4.0 / 3.0 * t54 * t118) / 8.0;
        let t127 = -t69 * t71 / 3.0 + t75 * t77 / 3.0 + t80 * t81 * t71 / 3.0 + t8 * t5 * t124;
        let tvrho0 = t66 * t127 + tzk0;
        vrho[ip] += tvrho0;
        let t129 = t66 * param_b;
        let t138 = t53 * t49;
        let t144 = -t12 * t18 + t35 * t12 * t31 / 8.0 + t40 * t12 * t44 / 144.0 - t48 * (4.0 / 3.0 * t56 - t138 * t37 / 2.0) / 8.0;
        let t145 = t76 * t144;
        let tvsigma0 = t129 * t145;
        vsigma[ip] += tvsigma0;
        let t149 = 1.0 / t67 / t4;
        let t150 = t149 * t91;
        let t154 = 1.0 / t1 / t9;
        let t157 = t74 * t154;
        let t160 = param_c * param_c;
        let t161 = param_b * t160;
        let t162 = t161 * t12;
        let t165 = t74 * t12;
        let t166 = t7 * t68;
        let t167 = t166 * t81;
        let t170 = t76 * t124;
        let t173 = t8 * t149;
        let t174 = t62 * t91;
        let t178 = t124 * param_d;
        let t185 = t9 * t9;
        let t187 = 1.0 / t10 / t185;
        let t188 = sigma[ip] * t187;
        let t193 = t91 * param_d;
        let t194 = t193 * t149;
        let t195 = 1.0 / t85;
        let t196 = t194 * t195;
        let t198 = t92 * t12;
        let t200 = t15 * t154;
        let t202 = -7.0 / 324.0 * t196 + 7.0 / 108.0 * t198 - 7.0 / 162.0 * t200;
        let t207 = -t196 / 81.0 + t198 / 27.0 - 2.0 / 81.0 * t200;
        let t208 = t207 * sigma[ip];
        let t213 = t187 * t31;
        let t219 = 2.0 / 9.0 * t196 - 2.0 / 3.0 * t198 + 4.0 / 9.0 * t200;
        let t220 = t219 * sigma[ip];
        let t225 = t187 * t44;
        let t231 = t49 * t187 * t31;
        let t237 = -88.0 / 9.0 * t188 * t18 + 16.0 / 3.0 * t88 * t98 - t13 * t202 + t208 * t37 / 8.0 - 2.0 / 3.0 * t101 * t104 + 11.0 / 9.0 * t36 * t213 + t220 * t45 / 144.0 - t109 * t112 / 27.0 + 11.0 / 162.0 * t41 * t225 - t48 * (352.0 / 27.0 * t50 * t213 - 44.0 / 9.0 * t54 * t231) / 8.0;
        let t240 = -2.0 / 9.0 * t150 * t12 + 4.0 / 9.0 * t69 * t154 - 4.0 / 9.0 * t157 * t77 + t162 * t77 / 9.0 + 2.0 / 9.0 * t165 * t167 + 2.0 / 3.0 * t75 * t170 + 2.0 / 9.0 * t173 * t174 * t12 + 2.0 / 3.0 * t80 * t178 * t71 - 4.0 / 9.0 * t80 * t81 * t154 + t8 * t5 * t237;
        let tv2rho20 = 2.0 * param_a * t127 + t66 * t240;
        v2rho2[ip] += tv2rho20;
        let t242 = param_a * param_b;
        let t245 = t2 * param_a * param_b;
        let t246 = param_c * t7;
        let t247 = t5 * t144;
        let t252 = t166 * t144 * param_d;
        let t276 = 8.0 / 3.0 * t87 * t18 - t12 * t98 + t100 * t12 * t31 / 8.0 - t35 * t87 * t31 / 3.0 + t108 * t12 * t44 / 144.0 - t40 * t87 * t44 / 54.0 - t48 * (-32.0 / 9.0 * t118 + 4.0 / 3.0 * t138 * t104) / 8.0;
        let t277 = t76 * t276;
        let tv2rhosigma0 = t242 * t145 + t245 * t246 * t247 / 3.0 + t245 * t252 / 3.0 + t129 * t277;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
        let t284 = 1.0 / t1 / t85;
        let t287 = t185 * rho[ip];
        let t289 = 1.0 / t10 / t287;
        let t290 = sigma[ip] * t289;
        let t297 = t91 * t91;
        let t298 = t67 * t67;
        let t299 = 1.0 / t298;
        let t300 = t297 * t299;
        let t302 = 1.0 / t1 / t185;
        let t303 = t300 * t302;
        let t305 = 1.0 / t185;
        let t306 = t194 * t305;
        let t308 = t92 * t87;
        let t310 = t15 * t284;
        let t312 = -7.0 / 324.0 * t303 + 35.0 / 324.0 * t306 - 91.0 / 486.0 * t308 + 49.0 / 486.0 * t310;
        let t318 = -t303 / 81.0 + 5.0 / 81.0 * t306 - 26.0 / 243.0 * t308 + 14.0 / 243.0 * t310;
        let t319 = t318 * sigma[ip];
        let t325 = t289 * t31;
        let t332 = 2.0 / 9.0 * t303 - 10.0 / 9.0 * t306 + 52.0 / 27.0 * t308 - 28.0 / 27.0 * t310;
        let t333 = t332 * sigma[ip];
        let t340 = t289 * t44;
        let t346 = t49 * t289 * t31;
        let t352 = 1232.0 / 27.0 * t290 * t18 - 88.0 / 3.0 * t188 * t98 + 8.0 * t88 * t202 - t13 * t312 + t319 * t37 / 8.0 - t208 * t104 + 11.0 / 3.0 * t101 * t213 - 154.0 / 27.0 * t36 * t325 + t333 * t45 / 144.0 - t220 * t112 / 18.0 + 11.0 / 54.0 * t109 * t225 - 77.0 / 243.0 * t41 * t340 - t48 * (-4928.0 / 81.0 * t50 * t325 + 616.0 / 27.0 * t54 * t346) / 8.0;
        let t355 = t299 * t193;
        let t358 = t74 * t284;
        let t361 = t74 * t87;
        let t364 = t161 * t305;
        let t367 = t166 * t178;
        let t370 = t74 * t305;
        let t371 = t7 * t149;
        let t372 = t371 * t174;
        let t386 = t161 * t87;
        let t391 = t160 * param_c;
        let t392 = param_b * t391;
        let t393 = t392 * t305;
        let t396 = t76 * t237;
        let t398 = t124 * t91;
        let t402 = t8 * t299;
        let t403 = t62 * t193;
        let t407 = t237 * param_d;
        let t410 = 8.0 / 9.0 * t150 * t87 - 28.0 / 27.0 * t69 * t284 + t8 * t5 * t352 - 2.0 / 9.0 * t355 * t305 + 28.0 / 27.0 * t358 * t77 - 8.0 / 9.0 * t361 * t167 + t364 * t167 / 9.0 + 2.0 / 3.0 * t165 * t367 + 2.0 / 9.0 * t370 * t372 - 8.0 / 9.0 * t173 * t174 * t87 - 4.0 / 3.0 * t80 * t178 * t154 + 28.0 / 27.0 * t80 * t81 * t284 - 4.0 / 3.0 * t157 * t170 - 4.0 / 9.0 * t386 * t77 + t162 * t170 / 3.0 + t393 * t77 / 27.0 + t75 * t396 + 2.0 / 3.0 * t173 * t398 * t12 + 2.0 / 9.0 * t402 * t403 * t305 + t80 * t407 * t71;
        let tv3rho30 = 3.0 * param_a * t240 + t66 * t410;
        v3rho3[ip] += tv3rho30;
        let t412 = t242 * param_c;
        let t413 = t71 * t7;
        let t417 = t242 * t7;
        let t418 = t68 * t144;
        let t419 = param_d * t71;
        let t425 = t94 * param_a;
        let t426 = t425 * param_b;
        let t427 = t160 * t7;
        let t431 = t425 * t74;
        let t434 = t5 * t276;
        let t439 = t371 * t144 * t91;
        let t443 = t166 * t276 * param_d;
        let t475 = -88.0 / 9.0 * t187 * t18 + 16.0 / 3.0 * t87 * t98 - t12 * t202 + t207 * t12 * t31 / 8.0 - 2.0 / 3.0 * t100 * t87 * t31 + 11.0 / 9.0 * t35 * t187 * t31 + t219 * t12 * t44 / 144.0 - t108 * t87 * t44 / 27.0 + 11.0 / 162.0 * t40 * t187 * t44 - t48 * (352.0 / 27.0 * t231 - 44.0 / 9.0 * t138 * t213) / 8.0;
        let t476 = t76 * t475;
        let tv3rho2sigma0 = 2.0 / 9.0 * t412 * t413 * t247 + 2.0 / 9.0 * t417 * t418 * t419 + 2.0 * t242 * t277 + t426 * t427 * t247 / 9.0 + 2.0 / 9.0 * t431 * t252 + 2.0 / 3.0 * t245 * t246 * t434 + 2.0 / 9.0 * t426 * t439 + 2.0 / 3.0 * t245 * t443 + t129 * t476;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rhosigma20 = 0.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3sigma30 = 0.0;
        v3sigma3[ip] += tv3sigma30;
        let t481 = 1.0 / t298 / t4;
        let t484 = 1.0 / t1 / t287;
        let t491 = 1.0 / t287;
        let t502 = 1.0 / t10 / t185 / t9;
        let t503 = t502 * t31;
        let t523 = t297 * param_d * t481 * t289;
        let t525 = t300 * t484;
        let t527 = t194 * t491;
        let t529 = t92 * t187;
        let t531 = t15 * t302;
        let t565 = -4.0 / 3.0 * t319 * t104 + 22.0 / 3.0 * t208 * t213 - 616.0 / 27.0 * t101 * t325 + 2618.0 / 81.0 * t36 * t503 - 2.0 / 27.0 * t333 * t112 + 11.0 / 27.0 * t220 * t225 - 308.0 / 243.0 * t109 * t340 + 1309.0 / 729.0 * t41 * t502 * t44 + 4928.0 / 27.0 * t290 * t98 - 176.0 / 3.0 * t188 * t202 + 32.0 / 3.0 * t88 * t312 - t13 * (-7.0 / 243.0 * t523 + 49.0 / 243.0 * t525 - 406.0 / 729.0 * t527 + 175.0 / 243.0 * t529 - 245.0 / 729.0 * t531) - 20944.0 / 81.0 * sigma[ip] * t502 * t18 + (-4.0 / 243.0 * t523 + 28.0 / 243.0 * t525 - 232.0 / 729.0 * t527 + 100.0 / 243.0 * t529 - 140.0 / 729.0 * t531) * sigma[ip] * t37 / 8.0 + (8.0 / 27.0 * t523 - 56.0 / 27.0 * t525 + 464.0 / 81.0 * t527 - 200.0 / 27.0 * t529 + 280.0 / 81.0 * t531) * sigma[ip] * t45 / 144.0 - t48 * (83776.0 / 243.0 * t50 * t503 - 10472.0 / 81.0 * t54 * t49 * t502 * t31) / 8.0;
        let t594 = t7 * t299;
        let t604 = -8.0 / 27.0 * t481 * t297 * t484 - 320.0 / 81.0 * t150 * t187 + 280.0 / 81.0 * t69 * t302 + 16.0 / 9.0 * t355 * t491 + t8 * t5 * t565 + 320.0 / 81.0 * t74 * t187 * t167 - 32.0 / 9.0 * t361 * t367 - 8.0 / 9.0 * t161 * t491 * t167 - 16.0 / 9.0 * t74 * t491 * t372 + 4.0 / 9.0 * t364 * t367 + 4.0 / 81.0 * t392 * t484 * t167 + 4.0 / 27.0 * t161 * t484 * t372 + 4.0 / 3.0 * t165 * t166 * t407 + 8.0 / 9.0 * t370 * t371 * t398 + 8.0 / 27.0 * t74 * t484 * t594 * t403 - 280.0 / 81.0 * t74 * t302 * t77 + 320.0 / 81.0 * t173 * t174 * t187;
        let t643 = t160 * t160;
        let t661 = 112.0 / 27.0 * t80 * t178 * t284 - 280.0 / 81.0 * t80 * t81 * t302 + 4.0 / 3.0 * t75 * t76 * t352 + 4.0 / 3.0 * t80 * t352 * param_d * t71 + 112.0 / 27.0 * t358 * t170 + 160.0 / 81.0 * t161 * t187 * t77 - 32.0 / 9.0 * t173 * t398 * t87 - 16.0 / 9.0 * t402 * t403 * t491 - 8.0 / 3.0 * t80 * t407 * t154 - 8.0 / 3.0 * t157 * t396 - 16.0 / 9.0 * t386 * t170 - 8.0 / 27.0 * t392 * t491 * t77 + 2.0 / 3.0 * t162 * t396 + 4.0 / 27.0 * t393 * t170 + param_b * t643 * t484 * t77 / 81.0 + 4.0 / 3.0 * t173 * t237 * t91 * t12 + 8.0 / 9.0 * t402 * t124 * t193 * t305 + 8.0 / 27.0 * t8 * t481 * t62 * t297 * t484;
        let tv4rho40 = 4.0 * param_a * t410 + t66 * (t604 + t661);
        v4rho4[ip] += tv4rho40;
        let t684 = t195 * param_a;
        let t685 = t684 * param_b;
        let t762 = 1232.0 / 27.0 * t289 * t18 - 88.0 / 3.0 * t187 * t98 + 8.0 * t87 * t202 - t12 * t312 + t318 * t12 * t31 / 8.0 - t207 * t87 * t31 + 11.0 / 3.0 * t100 * t187 * t31 - 154.0 / 27.0 * t35 * t289 * t31 + t332 * t12 * t44 / 144.0 - t219 * t87 * t44 / 18.0 + 11.0 / 54.0 * t108 * t187 * t44 - 77.0 / 243.0 * t40 * t289 * t44 - t48 * (-4928.0 / 81.0 * t346 + 616.0 / 27.0 * t138 * t325) / 8.0;
        let tv4rho3sigma0 = 2.0 / 3.0 * t412 * t413 * t434 - t242 * t160 * t12 * t7 * t247 / 9.0 + 2.0 / 3.0 * t417 * t68 * t276 * t419 - 2.0 / 9.0 * t417 * t149 * t144 * t91 * t12 + t426 * t427 * t434 / 3.0 + t685 * t391 * t7 * t247 / 27.0 + t245 * t246 * t5 * t475 + 2.0 / 3.0 * t426 * t371 * t276 * t91 + 2.0 / 9.0 * t685 * t594 * t144 * t193 + t245 * t166 * t475 * param_d - 8.0 / 27.0 * t412 * t154 * t7 * t247 - 2.0 / 9.0 * t242 * param_c * t12 * t252 - 8.0 / 27.0 * t417 * t418 * param_d * t154 + t684 * t161 * t252 / 9.0 + 2.0 / 3.0 * t431 * t443 + 2.0 / 9.0 * t684 * t74 * t439 + 3.0 * t242 * t476 + t129 * t76 * t762;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let tv4rho2sigma20 = 0.0;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rhosigma30 = 0.0;
        v4rhosigma3[ip] += tv4rhosigma30;
        let tv4sigma40 = 0.0;
        v4sigma4[ip] += tv4sigma40;
    }
}
