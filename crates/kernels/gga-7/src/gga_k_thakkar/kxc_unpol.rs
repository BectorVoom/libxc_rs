//! GGA_K_THAKKAR kxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 36 shared lines across all orders.
//! Delta: 76 lines unique to kxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_k_thakkar_kxc_unpol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (36 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = M_CBRT2;
        let t25 = t24 * t24;
        let t26 = sigma[ip] * t25;
        let t27 = rho[ip] * rho[ip];
        let t29 = 1.0 / t22 / t27;
        let t30 = f64::sqrt(sigma[ip]);
        let t31 = t30 * t24;
        let t33 = 1.0 / t21 / rho[ip];
        let t35 = f64::ln(t31 * t33 + f64::sqrt(pow_2(t31 * t33) + 1.0));
        let t36 = t33 * t35;
        let t39 = 1.0 + 0.253e-1 * t31 * t36;
        let t40 = 1.0 / t39;
        let t44 = M_CBRT4;
        let t49 = 2.0 * t44 * t30 * t24 * t33 + 1.0;
        let t50 = 1.0 / t49;
        let t51 = t33 * t50;
        let t54 = 1.0 + 0.55e-2 * t26 * t29 * t40 - 0.72e-1 * t31 * t51;
        let t58 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t54);
        let tzk0 = 2.0 * t58;
        zk[ip] += tzk0;
        // --- vxc delta (29 lines) ---
        let t60 = t20 / t21;
        let t64 = t27 * rho[ip];
        let t66 = 1.0 / t22 / t64;
        let t70 = t39 * t39;
        let t71 = 1.0 / t70;
        let t72 = t29 * t71;
        let t74 = 1.0 / t21 / t27;
        let t75 = t74 * t35;
        let t78 = t26 * t29;
        let t79 = t78 + 1.0;
        let t80 = f64::sqrt(t79);
        let t81 = 1.0 / t80;
        let t82 = t66 * t81;
        let t85 = -0.33733333333333333333e-1 * t31 * t75 - 0.33733333333333333333e-1 * t26 * t82;
        let t89 = t74 * t50;
        let t92 = t49 * t49;
        let t93 = 1.0 / t92;
        let t95 = t66 * t93 * t44;
        let t98 = -0.14666666666666666667e-1 * t26 * t66 * t40 - 0.55e-2 * t26 * t72 * t85 + 0.96e-1 * t31 * t89 - 0.192e0 * t26 * t95;
        let t103 = piecewise3(t2, 0.0, t7 * t60 * t54 / 10.0 + 3.0 / 20.0 * t7 * t23 * t98);
        let tvrho0 = 2.0 * rho[ip] * t103 + 2.0 * t58;
        vrho[ip] += tvrho0;
        let t106 = t25 * t29;
        let t109 = 1.0 / t30;
        let t110 = t109 * t24;
        let t115 = 0.1265e-1 * t110 * t36 + 0.1265e-1 * t106 * t81;
        let t121 = t93 * t44;
        let t124 = 0.55e-2 * t106 * t40 - 0.55e-2 * t26 * t72 * t115 - 0.36e-1 * t110 * t51 + 0.72e-1 * t106 * t121;
        let t128 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t124);
        let tvsigma0 = 2.0 * rho[ip] * t128;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (55 lines) ---
        let t131 = t20 * t33;
        let t138 = t27 * t27;
        let t140 = 1.0 / t22 / t138;
        let t144 = t66 * t71;
        let t149 = 1.0 / t70 / t39;
        let t150 = t29 * t149;
        let t151 = t85 * t85;
        let t156 = 1.0 / t21 / t64;
        let t157 = t156 * t35;
        let t160 = t140 * t81;
        let t163 = sigma[ip] * sigma[ip];
        let t164 = t163 * t24;
        let t165 = t138 * t64;
        let t167 = 1.0 / t21 / t165;
        let t169 = 1.0 / t80 / t79;
        let t173 = 0.7871111111111111111e-1 * t31 * t157 + 0.16866666666666666667e0 * t26 * t160 - 0.89955555555555555555e-1 * t164 * t167 * t169;
        let t177 = t156 * t50;
        let t181 = t140 * t93 * t44;
        let t184 = t30 * sigma[ip];
        let t185 = t138 * t27;
        let t186 = 1.0 / t185;
        let t189 = 1.0 / t92 / t49;
        let t190 = t44 * t44;
        let t191 = t189 * t190;
        let t194 = 0.53777777777777777779e-1 * t26 * t140 * t40 + 0.29333333333333333334e-1 * t26 * t144 * t85 + 0.11e-1 * t26 * t150 * t151 - 0.55e-2 * t26 * t72 * t173 - 0.224e0 * t31 * t177 + 0.96e0 * t26 * t181 - 0.2048e1 * t184 * t186 * t191;
        let t199 = piecewise3(t2, 0.0, -t7 * t131 * t54 / 30.0 + t7 * t60 * t98 / 5.0 + 3.0 / 20.0 * t7 * t23 * t194);
        let tv2rho20 = 2.0 * rho[ip] * t199 + 4.0 * t103;
        v2rho2[ip] += tv2rho20;
        let t205 = t25 * t66;
        let t208 = t71 * t85;
        let t214 = t149 * t115;
        let t215 = t214 * t85;
        let t223 = 1.0 / t21 / t185;
        let t224 = t24 * t223;
        let t225 = t169 * sigma[ip];
        let t228 = -0.16866666666666666667e-1 * t110 * t75 - 0.506e-1 * t205 * t81 + 0.33733333333333333333e-1 * t224 * t225;
        let t236 = t138 * rho[ip];
        let t237 = 1.0 / t236;
        let t239 = t190 * t30;
        let t242 = -0.14666666666666666667e-1 * t205 * t40 - 0.55e-2 * t106 * t208 + 0.14666666666666666667e-1 * t26 * t144 * t115 + 0.11e-1 * t78 * t215 - 0.55e-2 * t26 * t72 * t228 + 0.48e-1 * t110 * t89 - 0.288e0 * t205 * t121 + 0.768e0 * t237 * t189 * t239;
        let t247 = piecewise3(t2, 0.0, t7 * t60 * t124 / 10.0 + 3.0 / 20.0 * t7 * t23 * t242);
        let tv2rhosigma0 = 2.0 * rho[ip] * t247 + 2.0 * t128;
        v2rhosigma[ip] += tv2rhosigma0;
        let t250 = t71 * t115;
        let t253 = t115 * t115;
        let t257 = 1.0 / t184;
        let t258 = t257 * t24;
        let t261 = 1.0 / sigma[ip];
        let t262 = t261 * t25;
        let t263 = t29 * t81;
        let t267 = 1.0 / t21 / t236;
        let t271 = -0.6325e-2 * t258 * t36 + 0.6325e-2 * t262 * t263 - 0.1265e-1 * t24 * t267 * t169;
        let t278 = t29 * t93 * t44;
        let t281 = 1.0 / t138;
        let t286 = -0.11e-1 * t106 * t250 + 0.11e-1 * t26 * t150 * t253 - 0.55e-2 * t26 * t72 * t271 + 0.18e-1 * t258 * t51 + 0.36e-1 * t262 * t278 - 0.288e0 * t281 * t189 * t190 * t109;
        let t290 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t286);
        let tv2sigma20 = 2.0 * rho[ip] * t290;
        v2sigma2[ip] += tv2sigma20;
        // --- kxc delta (this level) (76 lines) ---
        let t293 = t20 * t74;
        let t304 = 1.0 / t22 / t236;
        let t308 = t140 * t71;
        let t312 = t66 * t149;
        let t319 = t70 * t70;
        let t320 = 1.0 / t319;
        let t321 = t29 * t320;
        let t322 = t151 * t85;
        let t326 = t149 * t85;
        let t327 = t326 * t173;
        let t331 = 1.0 / t21 / t138;
        let t332 = t331 * t35;
        let t338 = t138 * t138;
        let t340 = 1.0 / t21 / t338;
        let t344 = t163 * sigma[ip];
        let t345 = t338 * t64;
        let t346 = 1.0 / t345;
        let t348 = t79 * t79;
        let t350 = 1.0 / t80 / t348;
        let t353 = -0.26237037037037037037e0 * t31 * t332 - 0.89205925925925925928e0 * t26 * t304 * t81 + 0.11094518518518518519e1 * t164 * t340 * t169 - 0.71964444444444444444e0 * t344 * t346 * t350;
        let t357 = t331 * t50;
        let t364 = 1.0 / t165;
        let t369 = t92 * t92;
        let t370 = 1.0 / t369;
        let t371 = t370 * t24;
        let t374 = -0.25096296296296296297e0 * t26 * t304 * t40 - 0.16133333333333333334e0 * t26 * t308 * t85 - 0.88000000000000000001e-1 * t26 * t312 * t151 + 0.44000000000000000001e-1 * t26 * t144 * t173 - 0.33e-1 * t26 * t321 * t322 + 0.33e-1 * t78 * t327 - 0.55e-2 * t26 * t72 * t353 + 0.74666666666666666667e0 * t31 * t357 - 0.50773333333333333333e1 * t26 * t304 * t93 * t44 + 0.22528e2 * t184 * t364 * t191 - 0.65536e2 * t163 * t340 * t371;
        let t379 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t293 * t54 - t7 * t131 * t98 / 10.0 + 3.0 / 10.0 * t7 * t60 * t194 + 3.0 / 20.0 * t7 * t23 * t374);
        let tv3rho30 = 2.0 * rho[ip] * t379 + 6.0 * t199;
        v3rho3[ip] += tv3rho30;
        let t389 = t25 * t140;
        let t394 = t149 * t151;
        let t397 = t71 * t173;
        let t403 = t26 * t66;
        let t409 = t320 * t115;
        let t410 = t409 * t151;
        let t413 = t149 * t228;
        let t414 = t413 * t85;
        let t417 = t214 * t173;
        let t424 = t24 * t167;
        let t427 = t338 * t27;
        let t429 = 1.0 / t427 * t350;
        let t432 = 0.39355555555555555556e-1 * t110 * t157 + 0.20802222222222222222e0 * t389 * t81 - 0.34857777777777777777e0 * t424 * t225 + 0.26986666666666666666e0 * t429 * t163;
        let t443 = t167 * t370;
        let t444 = sigma[ip] * t24;
        let t447 = 0.53777777777777777779e-1 * t389 * t40 + 0.29333333333333333334e-1 * t205 * t208 + 0.11e-1 * t106 * t394 - 0.55e-2 * t106 * t397 - 0.53777777777777777779e-1 * t26 * t308 * t115 - 0.58666666666666666667e-1 * t403 * t215 + 0.29333333333333333334e-1 * t26 * t144 * t228 - 0.33e-1 * t78 * t410 + 0.22e-1 * t78 * t414 + 0.11e-1 * t78 * t417 - 0.55e-2 * t26 * t72 * t432 - 0.112e0 * t110 * t177 + 0.1184e1 * t389 * t121 - 0.6912e1 * t186 * t189 * t239 + 0.24576e2 * t443 * t444;
        let t452 = piecewise3(t2, 0.0, -t7 * t131 * t124 / 30.0 + t7 * t60 * t242 / 5.0 + 3.0 / 20.0 * t7 * t23 * t447);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t452 + 4.0 * t247;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t462 = t71 * t228;
        let t468 = t320 * t253;
        let t469 = t468 * t85;
        let t472 = t214 * t228;
        let t478 = t149 * t271;
        let t479 = t478 * t85;
        let t488 = t338 * rho[ip];
        let t490 = 1.0 / t488 * t350;
        let t493 = 0.84333333333333333333e-2 * t258 * t75 - 0.84333333333333333337e-2 * t262 * t82 + 0.84333333333333333334e-1 * t224 * t169 - 0.1012e0 * t490 * sigma[ip];
        let t504 = t223 * t370;
        let t507 = 0.29333333333333333333e-1 * t205 * t250 + 0.22e-1 * t106 * t215 - 0.11e-1 * t106 * t462 - 0.29333333333333333333e-1 * t26 * t312 * t253 - 0.33e-1 * t78 * t469 + 0.22e-1 * t78 * t472 + 0.14666666666666666667e-1 * t26 * t144 * t271 + 0.11e-1 * t78 * t479 - 0.55e-2 * t26 * t72 * t493 - 0.24e-1 * t258 * t89 - 0.48e-1 * t262 * t95 + 0.1536e1 * t109 * t237 * t191 - 0.9216e1 * t504 * t24;
        let t512 = piecewise3(t2, 0.0, t7 * t60 * t286 / 10.0 + 3.0 / 20.0 * t7 * t23 * t507);
        let tv3rhosigma20 = 2.0 * rho[ip] * t512 + 2.0 * t290;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t515 = t149 * t253;
        let t518 = t71 * t271;
        let t521 = t253 * t115;
        let t525 = t214 * t271;
        let t528 = t30 * t163;
        let t529 = 1.0 / t528;
        let t530 = t529 * t24;
        let t533 = 1.0 / t163;
        let t534 = t533 * t25;
        let t537 = t261 * t24;
        let t538 = t267 * t169;
        let t541 = 1.0 / t338;
        let t544 = 0.94875e-2 * t530 * t36 - 0.94875e-2 * t534 * t263 - 0.6325e-2 * t537 * t538 + 0.3795e-1 * t541 * t350;
        let t552 = t267 * t370;
        let t555 = 0.33e-1 * t106 * t515 - 0.165e-1 * t106 * t518 - 0.33e-1 * t26 * t321 * t521 + 0.33e-1 * t78 * t525 - 0.55e-2 * t26 * t72 * t544 - 0.27e-1 * t530 * t51 - 0.54e-1 * t534 * t278 + 0.3456e1 * t552 * t537;
        let t559 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t555);
        let tv3sigma30 = 2.0 * rho[ip] * t559;
        v3sigma3[ip] += tv3sigma30;
    }
}
