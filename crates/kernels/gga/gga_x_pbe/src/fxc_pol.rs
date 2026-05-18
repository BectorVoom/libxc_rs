//! GGA_X_PBE fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbe.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_pbe_fxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5::<f64>(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3::<f64>(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3::<f64>(t19);
        let t25 = piecewise3::<f64>(t20, t22, t23 * t19);
        let t26 = pow_1_3::<f64>(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = param_mu * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3::<f64>(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t33 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3::<f64>(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t43 = param_kappa + t29 * t34 * t39 / 24.0;
        let t48 = 1.0 + param_kappa * (1.0 - param_kappa / t43);
        let t52 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t48);
        let t53 = rho1 <= dens_threshold;
        let t54 = -t16;
        let t56 = piecewise5::<f64>(t14, t11, t10, t15, t54 * t7);
        let t57 = 1.0 + t56;
        let t58 = t57 <= zeta_threshold;
        let t59 = pow_1_3::<f64>(t57);
        let t61 = piecewise3::<f64>(t58, t22, t59 * t57);
        let t62 = t61 * t26;
        let t63 = t33 * sigma2;
        let t64 = rho1 * rho1;
        let t65 = pow_1_3::<f64>(rho1);
        let t66 = t65 * t65;
        let t68 = 1.0 / t66 / t64;
        let t72 = param_kappa + t29 * t63 * t68 / 24.0;
        let t77 = 1.0 + param_kappa * (1.0 - param_kappa / t72);
        let t81 = piecewise3::<f64>(t53, 0.0, -3.0 / 8.0 * t5 * t62 * t77);
        let tzk0 = t52 + t81;
        zk[ip] += tzk0;
        let t82 = t6 * t6;
        let t83 = 1.0 / t82;
        let t84 = t16 * t83;
        let t86 = piecewise5::<f64>(t10, 0.0, t14, 0.0, t7 - t84);
        let t89 = piecewise3::<f64>(t20, 0.0, 4.0 / 3.0 * t23 * t86);
        let t90 = t89 * t26;
        let t94 = t26 * t26;
        let t95 = 1.0 / t94;
        let t96 = t25 * t95;
        let t99 = t5 * t96 * t48 / 8.0;
        let t100 = param_kappa * param_kappa;
        let t101 = t27 * t100;
        let t102 = t5 * t101;
        let t103 = t43 * t43;
        let t105 = 1.0 / t103 * param_mu;
        let t106 = t105 * t28;
        let t107 = t35 * rho0;
        let t109 = 1.0 / t37 / t107;
        let t111 = t106 * t34 * t109;
        let t115 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t90 * t48 - t99 + t102 * t111 / 24.0);
        let t116 = t54 * t83;
        let t118 = piecewise5::<f64>(t14, 0.0, t10, 0.0, -t7 - t116);
        let t121 = piecewise3::<f64>(t58, 0.0, 4.0 / 3.0 * t59 * t118);
        let t122 = t121 * t26;
        let t126 = t61 * t95;
        let t129 = t5 * t126 * t77 / 8.0;
        let t131 = piecewise3::<f64>(t53, 0.0, -3.0 / 8.0 * t5 * t122 * t77 - t129);
        let tvrho0 = t52 + t81 + t6 * (t115 + t131);
        vrho[ip * 2] += tvrho0;
        let t135 = piecewise5::<f64>(t10, 0.0, t14, 0.0, -t7 - t84);
        let t138 = piecewise3::<f64>(t20, 0.0, 4.0 / 3.0 * t23 * t135);
        let t139 = t138 * t26;
        let t144 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t139 * t48 - t99);
        let t146 = piecewise5::<f64>(t14, 0.0, t10, 0.0, t7 - t116);
        let t149 = piecewise3::<f64>(t58, 0.0, 4.0 / 3.0 * t59 * t146);
        let t150 = t149 * t26;
        let t154 = t62 * t100;
        let t155 = t5 * t154;
        let t156 = t72 * t72;
        let t158 = 1.0 / t156 * param_mu;
        let t159 = t158 * t28;
        let t160 = t64 * rho1;
        let t162 = 1.0 / t66 / t160;
        let t164 = t159 * t63 * t162;
        let t168 = piecewise3::<f64>(t53, 0.0, -3.0 / 8.0 * t5 * t150 * t77 - t129 + t155 * t164 / 24.0);
        let tvrho1 = t52 + t81 + t6 * (t144 + t168);
        vrho[ip * 2 + 1] += tvrho1;
        let t171 = t28 * t33;
        let t173 = t105 * t171 * t39;
        let t176 = piecewise3::<f64>(t1, 0.0, -t102 * t173 / 64.0);
        let tvsigma0 = t6 * t176;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t178 = t158 * t171 * t68;
        let t181 = piecewise3::<f64>(t53, 0.0, -t155 * t178 / 64.0);
        let tvsigma2 = t6 * t181;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t184 = t23 * t23;
        let t185 = 1.0 / t184;
        let t186 = t86 * t86;
        let t189 = t82 * t6;
        let t190 = 1.0 / t189;
        let t191 = t16 * t190;
        let t194 = piecewise5::<f64>(t10, 0.0, t14, 0.0, -2.0 * t83 + 2.0 * t191);
        let t198 = piecewise3::<f64>(t20, 0.0, 4.0 / 9.0 * t185 * t186 + 4.0 / 3.0 * t23 * t194);
        let t199 = t198 * t26;
        let t203 = t89 * t95;
        let t205 = t5 * t203 * t48;
        let t208 = t5 * t90 * t100;
        let t212 = 1.0 / t94 / t6;
        let t213 = t25 * t212;
        let t216 = t5 * t213 * t48 / 12.0;
        let t218 = t5 * t96 * t100;
        let t219 = t218 * t111;
        let t223 = param_mu * param_mu;
        let t224 = 1.0 / t103 / t43 * t223;
        let t225 = t28 * t28;
        let t226 = t224 * t225;
        let t228 = 1.0 / t31 / t30;
        let t229 = sigma0 * sigma0;
        let t230 = t228 * t229;
        let t231 = t35 * t35;
        let t234 = 1.0 / t36 / t231 / t107;
        let t236 = t226 * t230 * t234;
        let t240 = 1.0 / t37 / t231;
        let t242 = t106 * t34 * t240;
        let t246 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t199 * t48 - t205 / 4.0 + t208 * t111 / 12.0 + t216 + t219 / 36.0 + t102 * t236 / 108.0 - 11.0 / 72.0 * t102 * t242);
        let t247 = t59 * t59;
        let t248 = 1.0 / t247;
        let t249 = t118 * t118;
        let t252 = t54 * t190;
        let t255 = piecewise5::<f64>(t14, 0.0, t10, 0.0, 2.0 * t83 + 2.0 * t252);
        let t259 = piecewise3::<f64>(t58, 0.0, 4.0 / 9.0 * t248 * t249 + 4.0 / 3.0 * t59 * t255);
        let t260 = t259 * t26;
        let t264 = t121 * t95;
        let t266 = t5 * t264 * t77;
        let t268 = t61 * t212;
        let t271 = t5 * t268 * t77 / 12.0;
        let t273 = piecewise3::<f64>(t53, 0.0, -3.0 / 8.0 * t5 * t260 * t77 - t266 / 4.0 + t271);
        let tv2rho20 = 2.0 * t115 + 2.0 * t131 + t6 * (t246 + t273);
        v2rho2[ip * 3] += tv2rho20;
        let t276 = t185 * t135;
        let t280 = piecewise5::<f64>(t10, 0.0, t14, 0.0, 2.0 * t191);
        let t284 = piecewise3::<f64>(t20, 0.0, 4.0 / 9.0 * t276 * t86 + 4.0 / 3.0 * t23 * t280);
        let t285 = t284 * t26;
        let t289 = t138 * t95;
        let t291 = t5 * t289 * t48;
        let t294 = t5 * t139 * t100;
        let t300 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t285 * t48 - t291 / 8.0 + t294 * t111 / 24.0 - t205 / 8.0 + t216 + t219 / 72.0);
        let t301 = t248 * t146;
        let t305 = piecewise5::<f64>(t14, 0.0, t10, 0.0, 2.0 * t252);
        let t309 = piecewise3::<f64>(t58, 0.0, 4.0 / 9.0 * t301 * t118 + 4.0 / 3.0 * t59 * t305);
        let t310 = t309 * t26;
        let t314 = t149 * t95;
        let t316 = t5 * t314 * t77;
        let t320 = t5 * t122 * t100;
        let t324 = t5 * t126 * t100;
        let t325 = t324 * t164;
        let t328 = piecewise3::<f64>(t53, 0.0, -3.0 / 8.0 * t5 * t310 * t77 - t316 / 8.0 - t266 / 8.0 + t271 + t320 * t164 / 24.0 + t325 / 72.0);
        let tv2rho21 = t115 + t131 + t144 + t168 + t6 * (t300 + t328);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t333 = t135 * t135;
        let t338 = piecewise5::<f64>(t10, 0.0, t14, 0.0, 2.0 * t83 + 2.0 * t191);
        let t342 = piecewise3::<f64>(t20, 0.0, 4.0 / 9.0 * t185 * t333 + 4.0 / 3.0 * t23 * t338);
        let t343 = t342 * t26;
        let t349 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t343 * t48 - t291 / 4.0 + t216);
        let t350 = t146 * t146;
        let t355 = piecewise5::<f64>(t14, 0.0, t10, 0.0, -2.0 * t83 + 2.0 * t252);
        let t359 = piecewise3::<f64>(t58, 0.0, 4.0 / 9.0 * t248 * t350 + 4.0 / 3.0 * t59 * t355);
        let t360 = t359 * t26;
        let t366 = t5 * t150 * t100;
        let t372 = 1.0 / t156 / t72 * t223;
        let t373 = t372 * t225;
        let t374 = sigma2 * sigma2;
        let t375 = t228 * t374;
        let t376 = t64 * t64;
        let t379 = 1.0 / t65 / t376 / t160;
        let t381 = t373 * t375 * t379;
        let t385 = 1.0 / t66 / t376;
        let t387 = t159 * t63 * t385;
        let t391 = piecewise3::<f64>(t53, 0.0, -3.0 / 8.0 * t5 * t360 * t77 - t316 / 4.0 + t366 * t164 / 12.0 + t271 + t325 / 36.0 + t155 * t381 / 108.0 - 11.0 / 72.0 * t155 * t387);
        let tv2rho22 = 2.0 * t144 + 2.0 * t168 + t6 * (t349 + t391);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t397 = t218 * t173 / 192.0;
        let t398 = t231 * t35;
        let t400 = 1.0 / t36 / t398;
        let t403 = t226 * t228 * t400 * sigma0;
        let t407 = t105 * t171 * t109;
        let t411 = piecewise3::<f64>(t1, 0.0, -t208 * t173 / 64.0 - t397 - t102 * t403 / 288.0 + t102 * t407 / 24.0);
        let tv2rhosigma0 = t6 * t411 + t176;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let t416 = t324 * t178 / 192.0;
        let t418 = piecewise3::<f64>(t53, 0.0, -t320 * t178 / 64.0 - t416);
        let tv2rhosigma2 = t6 * t418 + t181;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t423 = piecewise3::<f64>(t1, 0.0, -t294 * t173 / 64.0 - t397);
        let tv2rhosigma3 = t6 * t423 + t176;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t427 = t376 * t64;
        let t429 = 1.0 / t65 / t427;
        let t432 = t373 * t228 * t429 * sigma2;
        let t436 = t158 * t171 * t162;
        let t440 = piecewise3::<f64>(t53, 0.0, -t366 * t178 / 64.0 - t416 - t155 * t432 / 288.0 + t155 * t436 / 24.0);
        let tv2rhosigma5 = t6 * t440 + t181;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t442 = t225 * t228;
        let t443 = t231 * rho0;
        let t447 = t224 * t442 / t36 / t443;
        let t450 = piecewise3::<f64>(t1, 0.0, t102 * t447 / 768.0);
        let tv2sigma20 = t6 * t450;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = 0.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = 0.0;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = 0.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = 0.0;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let t451 = t376 * rho1;
        let t455 = t372 * t442 / t65 / t451;
        let t458 = piecewise3::<f64>(t53, 0.0, t155 * t455 / 768.0);
        let tv2sigma25 = t6 * t458;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
    }
}
