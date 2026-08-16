//! GGA_C_LM lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lm.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_lm_lxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    param_lm_f: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = 1.0 / M_PI;
        let t2 = 1.0 / rho[ip];
        let t5 = 1.0 + t1 * t2 / 36000.0;
        let t6 = M_CBRT3;
        let t7 = t6 * t6;
        let t8 = pow_1_3::<f64>(t1);
        let t9 = 1.0 / t8;
        let t10 = t7 * t9;
        let t11 = M_CBRT4;
        let t12 = pow_1_3::<f64>(rho[ip]);
        let t14 = t10 * t11 * t12;
        let t16 = 1.0 + 10.0 * t14;
        let t17 = f64::ln(t16);
        let t19 = 0.252e-1 * t5 * t17;
        let t20 = t8 * t8;
        let t21 = t7 * t20;
        let t22 = t12 * t12;
        let t23 = 1.0 / t22;
        let t24 = t11 * t23;
        let t25 = t21 * t24;
        let t26 = 0.7e-5 * t25;
        let t27 = t6 * t8;
        let t28 = t11 * t11;
        let t31 = t27 * t28 / t12;
        let t32 = 0.105e-3 * t31;
        let t33 = 1.0 <= zeta_threshold;
        let t34 = pow_1_3::<f64>(zeta_threshold);
        let t36 = piecewise3::<f64>(t33, t34 * zeta_threshold, 1.0);
        let t39 = M_CBRT2;
        let t43 = (2.0 * t36 - 2.0) / (2.0 * t39 - 2.0);
        let t45 = 1.0 + 0.56588424210451674939e-6 * t2;
        let t47 = 1.0 + 25.0 * t14;
        let t48 = f64::ln(t47);
        let t54 = t43 * (-0.127e-1 * t45 * t48 - 0.64355555555555555556e-5 * t25 + 0.83833333333333333334e-4 * t31 - 0.41666666666666666667e-2 + t19);
        let t55 = M_PI * t7;
        let t56 = M_PI * M_PI;
        let t57 = pow_1_3::<f64>(t56);
        let t59 = 1.0 / t57 / t56;
        let t60 = rho[ip] * rho[ip];
        let t62 = 1.0 / t22 / t60;
        let t63 = sigma[ip] * t62;
        let t66 = t34 * t34;
        let t68 = piecewise3::<f64>(t33, t66 * zeta_threshold, 1.0);
        let t69 = f64::sqrt(t68);
        let t70 = 1.0 / t69;
        let t72 = f64::powf(t1, 1.0 / 6.0);
        let t73 = 1.0 / t72;
        let t74 = f64::sqrt(sigma[ip]);
        let t75 = t73 * t74;
        let t76 = f64::powf(rho[ip], 1.0 / 6.0);
        let t81 = f64::exp(-t6 * param_lm_f * t75 / t76 / rho[ip]);
        let t82 = t70 * t81;
        let t86 = t59 * (-7.0 / 9.0 * t63 * t36 + 2.0 * t82 * t63);
        let t89 = t55 * t86 * t12 / 144.0;
        let tzk0 = -t19 + t26 - t32 + 0.84e-2 + t54 + t89;
        zk[ip] += tzk0;
        let t90 = 1.0 / t60;
        let t92 = t1 * t90 * t17;
        let t93 = 0.7e-6 * t92;
        let t95 = t5 * t7 * t9;
        let t96 = 1.0 / t16;
        let t98 = t95 * t24 * t96;
        let t99 = 0.84e-1 * t98;
        let t101 = 1.0 / t22 / rho[ip];
        let t102 = t11 * t101;
        let t103 = t21 * t102;
        let t105 = t12 * rho[ip];
        let t107 = t28 / t105;
        let t108 = t27 * t107;
        let t113 = t45 * t7 * t9;
        let t114 = 1.0 / t47;
        let t121 = t43 * (0.71867298747273627173e-8 * t90 * t48 - 0.10583333333333333333e0 * t113 * t24 * t114 + 0.42903703703703703704e-5 * t103 - 0.27944444444444444445e-4 * t108 - t93 + t99);
        let t122 = t60 * rho[ip];
        let t124 = 1.0 / t22 / t122;
        let t125 = sigma[ip] * t124;
        let t129 = t70 * t6 * param_lm_f;
        let t130 = t74 * sigma[ip];
        let t131 = t73 * t130;
        let t132 = t60 * t60;
        let t133 = t76 * t76;
        let t134 = t133 * t133;
        let t135 = t134 * t76;
        let t138 = 1.0 / t135 / t132 * t81;
        let t145 = t59 * (56.0 / 27.0 * t125 * t36 + 7.0 / 3.0 * t129 * t131 * t138 - 16.0 / 3.0 * t82 * t125);
        let t147 = t55 * t145 * t12;
        let t150 = t55 * t86 * t23;
        let tvrho0 = -t19 + t26 - t32 + 0.84e-2 + t54 + t89 + rho[ip] * (t93 - t99 - 0.46666666666666666667e-5 * t103 + 0.35e-4 * t108 + t121 + t147 / 144.0 + t150 / 432.0);
        vrho[ip] += tvrho0;
        let t154 = t105 * M_PI;
        let t155 = t7 * t59;
        let t160 = 1.0 / t135 / t122 * t81;
        let t165 = -7.0 / 9.0 * t62 * t36 - t129 * t75 * t160 + 2.0 * t82 * t62;
        let tvsigma0 = t154 * t155 * t165 / 144.0;
        vsigma[ip] += tvsigma0;
        let t175 = 1.0 / t122;
        let t177 = t1 * t175 * t17;
        let t178 = 0.14e-5 * t177;
        let t181 = t9 * t11;
        let t182 = t181 * t96;
        let t183 = t1 * t62 * t7 * t182;
        let t184 = 0.46666666666666666666e-5 * t183;
        let t186 = t95 * t102 * t96;
        let t187 = 0.56e-1 * t186;
        let t189 = 1.0 / t20;
        let t190 = t5 * t6 * t189;
        let t191 = t16 * t16;
        let t192 = 1.0 / t191;
        let t194 = t190 * t107 * t192;
        let t195 = 0.84e0 * t194;
        let t196 = t11 * t62;
        let t197 = t21 * t196;
        let t201 = t28 / t12 / t60;
        let t202 = t27 * t201;
        let t207 = t181 * t114;
        let t214 = t45 * t6 * t189;
        let t215 = t47 * t47;
        let t216 = 1.0 / t215;
        let t223 = t43 * (-0.14373459749454725435e-7 * t175 * t48 + 0.11977883124545604529e-6 * t62 * t7 * t207 + 0.70555555555555555553e-1 * t113 * t102 * t114 + 0.26458333333333333332e1 * t214 * t107 * t216 - 0.7150617283950617284e-5 * t197 + 0.3725925925925925926e-4 * t202 + t178 - t184 - t187 - t195);
        let t225 = 1.0 / t22 / t132;
        let t226 = sigma[ip] * t225;
        let t229 = t132 * rho[ip];
        let t232 = 1.0 / t135 / t229 * t81;
        let t237 = param_lm_f * param_lm_f;
        let t238 = t70 * t7 * t237;
        let t239 = sigma[ip] * sigma[ip];
        let t240 = t9 * t239;
        let t241 = t132 * t122;
        let t242 = 1.0 / t241;
        let t243 = t242 * t81;
        let t250 = t59 * (-616.0 / 81.0 * t226 * t36 - 35.0 / 2.0 * t129 * t131 * t232 + 49.0 / 18.0 * t238 * t240 * t243 + 176.0 / 9.0 * t82 * t226);
        let t252 = t55 * t250 * t12;
        let t255 = t55 * t145 * t23;
        let t258 = t55 * t86 * t101;
        let tv2rho20 = 0.14e-5 * t92 - 0.168e0 * t98 - 0.93333333333333333334e-5 * t103 + 0.7e-4 * t108 + 2.0 * t121 + t147 / 72.0 + t150 / 216.0 + rho[ip] * (-t178 + t184 + t187 + t195 + 0.77777777777777777778e-5 * t197 - 0.46666666666666666667e-4 * t202 + t223 + t252 / 144.0 + t255 / 216.0 - t258 / 648.0);
        v2rho2[ip] += tv2rho20;
        let t262 = t59 * t165;
        let t271 = t9 * sigma[ip];
        let t272 = t132 * t60;
        let t273 = 1.0 / t272;
        let t274 = t273 * t81;
        let t280 = 56.0 / 27.0 * t124 * t36 + 37.0 / 6.0 * t129 * t75 * t138 - 7.0 / 6.0 * t238 * t271 * t274 - 16.0 / 3.0 * t82 * t124;
        let tv2rhosigma0 = t55 * t262 * t12 / 108.0 + t154 * t155 * t280 / 144.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t284 = 1.0 / t74;
        let t285 = t73 * t284;
        let t289 = 1.0 / t229;
        let t295 = t155 * (-3.0 / 2.0 * t129 * t285 * t160 + t238 * t9 * t289 * t81 / 2.0);
        let tv2sigma20 = t154 * t295 / 144.0;
        v2sigma2[ip] += tv2sigma20;
        let t307 = 1.0 / t132;
        let t309 = t1 * t307 * t17;
        let t310 = 0.42e-5 * t309;
        let t313 = t1 * t124 * t7 * t182;
        let t314 = 0.18666666666666666666e-4 * t313;
        let t316 = 1.0 / t12 / t122;
        let t319 = t189 * t28;
        let t320 = t319 * t192;
        let t321 = t1 * t316 * t6 * t320;
        let t322 = 0.69999999999999999999e-4 * t321;
        let t324 = t95 * t196 * t96;
        let t325 = 0.93333333333333333334e-1 * t324;
        let t327 = t190 * t201 * t192;
        let t328 = 0.168e1 * t327;
        let t329 = t5 * M_PI;
        let t331 = 1.0 / t191 / t16;
        let t333 = t329 * t90 * t331;
        let t334 = 0.672e2 * t333;
        let t335 = t11 * t124;
        let t336 = t21 * t335;
        let t338 = t28 * t316;
        let t339 = t27 * t338;
        let t347 = t319 * t216;
        let t358 = 1.0 / t215 / t47;
        let t363 = 0.43120379248364176305e-7 * t307 * t48 - 0.47911532498182418116e-6 * t124 * t7 * t207 - 0.44917061717046016982e-5 * t316 * t6 * t347 - 0.11759259259259259259e0 * t113 * t196 * t114 - 0.52916666666666666664e1 * t214 * t201 * t216 - 0.1662426112524598922e4 * t45 * t90 * t358 + 0.19068312757201646091e-4 * t336 - 0.86938271604938271607e-4 * t339 - t310 + t314 + t322 + t325 + t328 + t334;
        let t364 = t43 * t363;
        let t366 = 1.0 / t22 / t229;
        let t367 = sigma[ip] * t366;
        let t372 = 1.0 / t135 / t272 * t81;
        let t376 = t132 * t132;
        let t378 = 1.0 / t376 * t81;
        let t384 = f64::sqrt(t1);
        let t386 = t70 * t237 * param_lm_f / t384;
        let t387 = t74 * t239;
        let t388 = t376 * rho[ip];
        let t390 = 1.0 / t76 / t388;
        let t398 = t59 * (8624.0 / 243.0 * t367 * t36 + 13489.0 / 108.0 * t129 * t131 * t372 - 1421.0 / 36.0 * t238 * t240 * t378 + 343.0 / 36.0 * t386 * t387 * t390 * t81 - 2464.0 / 27.0 * t82 * t367);
        let t400 = t55 * t398 * t12;
        let t403 = t55 * t250 * t23;
        let t406 = t55 * t145 * t101;
        let t409 = t55 * t86 * t62;
        let t411 = t310 - t314 - t322 - t325 - t328 - t334 - 0.20740740740740740741e-4 * t336 + 0.10888888888888888889e-3 * t339 + t364 + t400 / 144.0 + t403 / 144.0 - t406 / 216.0 + 5.0 / 1944.0 * t409;
        let tv3rho30 = -0.42e-5 * t177 + 0.14e-4 * t183 + 0.168e0 * t186 + 0.252e1 * t194 + 0.23333333333333333334e-4 * t197 - 0.14e-3 * t202 + 3.0 * t223 + t252 / 48.0 + t255 / 72.0 - t258 / 216.0 + rho[ip] * t411;
        v3rho3[ip] += tv3rho30;
        let t413 = t59 * t280;
        let t429 = 1.0 / t76 / t376;
        let t436 = -616.0 / 81.0 * t225 * t36 - 1297.0 / 36.0 * t129 * t75 * t232 + 511.0 / 36.0 * t238 * t271 * t243 - 49.0 / 12.0 * t386 * t130 * t429 * t81 + 176.0 / 9.0 * t82 * t225;
        let tv3rho2sigma0 = t55 * t413 * t12 / 54.0 + t55 * t262 * t23 / 324.0 + t154 * t155 * t436 / 144.0;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t440 = t12 * M_PI;
        let t451 = 1.0 / t76 / t241;
        let t457 = t155 * (23.0 / 4.0 * t129 * t285 * t138 - 17.0 / 4.0 * t238 * t9 * t273 * t81 + 7.0 / 4.0 * t386 * t451 * t74 * t81);
        let tv3rhosigma20 = t440 * t295 / 108.0 + t154 * t457 / 144.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t460 = 1.0 / t130;
        let t461 = t73 * t460;
        let t464 = 1.0 / sigma[ip];
        let t465 = t9 * t464;
        let t466 = t289 * t81;
        let t470 = 1.0 / t76 / t272;
        let t476 = t155 * (-3.0 / 4.0 * t386 * t470 * t284 * t81 + 3.0 / 4.0 * t129 * t461 * t160 + 3.0 / 4.0 * t238 * t465 * t466);
        let tv3sigma30 = t154 * t476 / 144.0;
        v3sigma3[ip] += tv3sigma30;
        let t479 = t21 * t11 * t225;
        let t482 = 1.0 / t12 / t132;
        let t484 = t27 * t28 * t482;
        let t488 = 0.168e-4 * t1 * t289 * t17;
        let t491 = 0.2688e3 * t329 * t175 * t331;
        let t494 = sigma[ip] / t22 / t272;
        let t508 = t376 * t60;
        let t515 = t237 * t237;
        let t517 = t70 * t515 * t189;
        let t523 = t6 * t81;
        let t548 = 0.74666666666666666666e-2 * t307 * t331;
        let t554 = 0.85037037037037037035e-4 * t1 * t225 * t7 * t182;
        let t558 = 0.46666666666666666666e-3 * t1 * t482 * t6 * t320;
        let t561 = 0.48533333333333333333e1 * t190 * t338 * t192;
        let t566 = t215 * t215;
        let t574 = 0.24888888888888888889e0 * t95 * t335 * t96;
        let t576 = t191 * t191;
        let t581 = 672.0 * t329 * t62 / t576 * t7 * t181;
        let t585 = -0.69917146776406035667e-4 * t479 + 0.28979423868312757202e-3 * t484 + t488 - t491 + 0.21826364804727546031e-5 * t225 * t7 * t207 + 0.29944707811364011322e-4 * t482 * t6 * t347 - 0.17248151699345670522e-6 * t289 * t48 + 0.66497044500983956879e4 * t45 * t175 * t358 - t548 + 0.37629629629629629629e-2 * t307 * t358 - t554 - t558 - t561 + 0.15287037037037037037e2 * t214 * t338 * t216 + 0.4156065281311497305e5 * t45 * t62 / t566 * t10 * t11 - t574 - t581 + 0.31358024691358024691e0 * t113 * t335 * t114;
        let t599 = 0.76049382716049382717e-4 * t479 - 0.36296296296296296297e-3 * t484 - t488 + t491 + t55 * t59 * (-146608.0 / 729.0 * t494 * t36 - 207347.0 / 216.0 * t129 * t131 / t135 / t241 * t81 + 299047.0 / 648.0 * t238 * t240 / t388 * t81 - 24353.0 / 108.0 * t386 * t387 / t76 / t508 * t81 + 2401.0 / 216.0 * t517 * t239 * sigma[ip] / t12 / t376 / t122 * t523 + 41888.0 / 81.0 * t82 * t494) * t12 / 144.0 + t43 * t585 + t548 + t554 + t558 + t561 + t55 * t398 * t23 / 108.0 - t55 * t250 * t101 / 108.0 + 5.0 / 486.0 * t55 * t145 * t62 - 5.0 / 729.0 * t55 * t86 * t124 + t574 + t581;
        let tv4rho40 = rho[ip] * t599 - 0.74666666666666666666e-4 * t313 - 0.28e-3 * t321 - 0.672e1 * t327 + t403 / 36.0 - t406 / 54.0 + 5.0 / 486.0 * t409 + t400 / 36.0 - 0.37333333333333333333e0 * t324 + 4.0 * t364 + 0.168e-4 * t309 - 0.2688e3 * t333 - 0.82962962962962962965e-4 * t336 + 0.43555555555555555556e-3 * t339;
        v4rho4[ip] += tv4rho40;
        let tv4rho3sigma0 = t55 * t59 * t436 * t12 / 36.0 + t55 * t413 * t23 / 108.0 - t55 * t262 * t101 / 486.0 + t154 * t155 * (8624.0 / 243.0 * t366 * t36 + 50323.0 / 216.0 * t129 * t75 * t372 - 30541.0 / 216.0 * t238 * t271 * t378 + 2989.0 / 36.0 * t386 * t130 * t390 * t81 - 343.0 / 72.0 * t517 * t239 / t12 / t508 * t523 - 2464.0 / 27.0 * t82 * t366) / 144.0;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let tv4rho2sigma20 = t23 * M_PI * t295 / 324.0 + t440 * t457 / 54.0 + t154 * t155 * (-667.0 / 24.0 * t129 * t285 * t232 + 773.0 / 24.0 * t238 * t9 * t242 * t81 - 329.0 / 12.0 * t386 * t429 * t74 * t81 + 49.0 / 24.0 * t517 / t12 / t388 * sigma[ip] * t523) / 144.0;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rhosigma30 = t440 * t476 / 108.0 + t154 * t155 * (-23.0 / 8.0 * t129 * t461 * t138 - 23.0 / 8.0 * t238 * t465 * t274 + 29.0 / 4.0 * t386 * t284 * t451 * t81 - 7.0 / 8.0 * t517 / t12 / t376 * t6 * t81) / 144.0;
        v4rhosigma3[ip] += tv4rhosigma30;
        let tv4sigma40 = t154 * t155 * (-9.0 / 8.0 * t129 * t73 / t387 * t160 - 9.0 / 8.0 * t238 * t9 / t239 * t466 - 3.0 / 4.0 * t386 * t460 * t470 * t81 + 3.0 / 8.0 * t517 / t12 / t241 * t464 * t523) / 144.0;
        v4sigma4[ip] += tv4sigma40;
    }
}
