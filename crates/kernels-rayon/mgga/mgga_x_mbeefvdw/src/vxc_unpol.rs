//! MGGA_X_MBEEFVDW vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mbeefvdw.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mbeefvdw_vxc_unpol(
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
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = t11 + 1.0;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = t26 * sigma[ip];
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t19 * t19;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t35 = sigma[ip] * t29;
        let t36 = t35 * t33;
        let t39 = 6.5124 + t26 * t36 / 24.0;
        let t40 = 1.0 / t39;
        let t41 = t34 * t40;
        let t42 = t27 * t41;
        let t44 = t42 / 12.0 - 1.0;
        let t45 = tau[ip] * t29;
        let t47 = 1.0 / t31 / rho[ip];
        let t53 = 5.0 / 9.0 * (t45 * t47 - t36 / 8.0) * t21 * t25;
        let t54 = 10000.0 <= t53;
        let t55 = 10000.0 < t53;
        let t56 = piecewise3(t55, t53, 10000.0);
        let t57 = t56 * t56;
        let t60 = t57 * t56;
        let t61 = 1.0 / t60;
        let t62 = t57 * t57;
        let t63 = 1.0 / t62;
        let t66 = piecewise3(t55, 10000.0, t53);
        let t67 = t66 * t66;
        let t68 = 1.0 - t67;
        let t69 = t68 * t68;
        let t70 = t69 * t68;
        let t71 = t67 * t66;
        let t72 = 1.0 + t71;
        let t74 = t71 * t72 + 1.0;
        let t75 = 1.0 / t74;
        let t77 = piecewise3(t54, 1.0 - 3.0 / t57 - t61 + 3.0 * t63, -t70 * t75);
        let t78 = t77 * t77;
        let t79 = t78 * t78;
        let t82 = 3.0 / 8.0 + 35.0 / 8.0 * t79 - 15.0 / 4.0 * t78;
        let t85 = t78 * t77;
        let t88 = 5.0 / 2.0 * t85 - 3.0 / 2.0 * t77;
        let t92 = -1.0 / 2.0 + 3.0 / 2.0 * t78;
        let t95 = t44 * t77;
        let t99 = t44 * t44;
        let t100 = t99 * t99;
        let t106 = 3.0 / 8.0 + 35.0 / 8.0 * t100 - 15.0 / 4.0 * t99;
        let t113 = -1.00478906e-07 * t44 * t82 - 0.00608338264 * t44 * t88 + 0.0318024096 * t44 * t92 + 0.0453837246 * t95 - 0.06972770593 * t77 + 0.0217681859775 * t78 + 0.00618699843125 * t100 + 0.01214700985 * t42 - 0.0851282539125 * t99 - 3.40722258e-09 * t106 * t82 + 5.74317889e-08 * t106 * t88 - 5.00749348e-07 * t106 * t92;
        let t114 = t106 * t77;
        let t116 = t99 * t44;
        let t119 = 5.0 / 2.0 * t116 - t42 / 8.0 + 3.0 / 2.0;
        let t126 = t119 * t77;
        let t129 = -1.0 / 2.0 + 3.0 / 2.0 * t99;
        let t136 = t129 * t77;
        let t141 = 1.0451438955835 + 9.19317034e-07 * t114 + 3.97324768e-09 * t119 * t82 - 5.49909413e-08 * t119 * t88 + 1.33707403e-07 * t119 * t92 + 0.0192374554 * t126 + 2.01895739e-07 * t129 * t82 - 6.57949254e-07 * t129 * t88 - 0.00521818079 * t129 * t92 - 0.0222650139 * t136 + 0.00061919587625 * t79 - 0.050282912 * t116 + 0.00351985355 * t85;
        let t142 = t113 + t141;
        let t146 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t142);
        let tzk0 = 2.0 * t146;
        zk[ip] += tzk0;
        let t148 = t18 / t31;
        let t154 = t30 * rho[ip];
        let t156 = 1.0 / t31 / t154;
        let t162 = 5.0 / 9.0 * (-5.0 / 3.0 * t45 * t33 + t35 * t156 / 3.0) * t21 * t25;
        let t163 = piecewise3(t55, t162, 0.0);
        let t166 = t63 * t163;
        let t169 = 1.0 / t62 / t56;
        let t170 = t169 * t163;
        let t173 = t69 * t75;
        let t174 = piecewise3(t55, 0.0, t162);
        let t175 = t66 * t174;
        let t178 = t74 * t74;
        let t179 = 1.0 / t178;
        let t180 = t70 * t179;
        let t181 = t67 * t72;
        let t183 = t67 * t67;
        let t184 = t183 * t66;
        let t187 = 3.0 * t181 * t174 + 3.0 * t184 * t174;
        let t190 = piecewise3(t54, 6.0 * t61 * t163 + 3.0 * t166 - 12.0 * t170, 6.0 * t173 * t175 + t180 * t187);
        let t192 = t29 * t156;
        let t193 = t192 * t40;
        let t194 = t27 * t193;
        let t196 = t21 * t21;
        let t198 = 1.0 / t23 / t22;
        let t199 = t196 * t198;
        let t200 = sigma[ip] * sigma[ip];
        let t201 = t199 * t200;
        let t202 = t30 * t30;
        let t203 = t202 * t30;
        let t205 = 1.0 / t19 / t203;
        let t207 = t39 * t39;
        let t208 = 1.0 / t207;
        let t209 = t28 * t205 * t208;
        let t210 = t201 * t209;
        let t216 = -2.0 / 9.0 * t194 + t210 / 54.0;
        let t217 = t44 * t216;
        let t232 = t216 * t92;
        let t234 = t216 * t77;
        let t236 = t44 * t190;
        let t238 = t216 * t82;
        let t240 = t85 * t190;
        let t242 = t77 * t190;
        let t246 = 35.0 / 2.0 * t240 - 15.0 / 2.0 * t242;
        let t249 = t216 * t88;
        let t251 = t78 * t190;
        let t253 = -0.06972770593 * t190 - 0.032392026266666665 * t194 + 0.0026993355222222223 * t210 + 4.01122209e-07 * t126 * t190 + 6.05687217e-07 * t217 * t82 - 1.973847762e-06 * t217 * t88 - 0.01565454237 * t217 * t92 - 0.01565454237 * t136 * t190 - 0.0667950417 * t217 * t77 + 0.0954072288 * t95 * t190 - 1.502248044e-06 * t114 * t190 + 0.0318024096 * t232 + 0.0453837246 * t234 + 0.0453837246 * t236 - 1.00478906e-07 * t238 + 0.002476783505 * t240 + 0.043536371955 * t242 - 1.00478906e-07 * t44 * t246 - 0.00608338264 * t249 + 0.01055956065 * t251;
        let t256 = 15.0 / 2.0 * t251 - 3.0 / 2.0 * t190;
        let t261 = t129 * t190;
        let t263 = t99 * t216;
        let t268 = 15.0 / 2.0 * t263 + t194 / 3.0 - t210 / 36.0;
        let t271 = t268 * t77;
        let t273 = t119 * t190;
        let t279 = t116 * t216;
        let t282 = 35.0 / 2.0 * t279 - 15.0 / 2.0 * t217;
        let t300 = t282 * t77;
        let t302 = t106 * t190;
        let t306 = 3.97324768e-09 * t268 * t82 + 3.97324768e-09 * t119 * t246 - 5.49909413e-08 * t268 * t88 - 5.49909413e-08 * t119 * t256 + 5.74317889e-08 * t106 * t256 + 0.024747993725 * t279 - 0.170256507825 * t217 - 5.00749348e-07 * t282 * t92 + 9.19317034e-07 * t300 + 9.19317034e-07 * t302 - 3.40722258e-09 * t282 * t82;
        let t308 = t253 - 0.00608338264 * t44 * t256 - 6.57949254e-07 * t129 * t256 - 0.0222650139 * t261 - 0.150848736 * t263 + 1.33707403e-07 * t268 * t92 + 0.0192374554 * t271 + 0.0192374554 * t273 + 2.01895739e-07 * t129 * t246 - 3.40722258e-09 * t106 * t246 + 5.74317889e-08 * t282 * t88 + t306;
        let t313 = piecewise3(t3, 0.0, -t7 * t148 * t142 / 8.0 - 3.0 / 8.0 * t7 * t20 * t308);
        let tvrho0 = 2.0 * rho[ip] * t313 + 2.0 * t146;
        vrho[ip] += tvrho0;
        let t316 = t26 * t41;
        let t319 = t202 * rho[ip];
        let t323 = t28 / t19 / t319 * t208;
        let t324 = t199 * sigma[ip] * t323;
        let t326 = t316 / 12.0 - t324 / 144.0;
        let t327 = t326 * t92;
        let t329 = t326 * t77;
        let t331 = t26 * t34;
        let t332 = 5.0 / 72.0 * t331;
        let t333 = piecewise3(t55, -t332, 0.0);
        let t336 = t63 * t333;
        let t338 = t169 * t333;
        let t341 = piecewise3(t55, 0.0, -t332);
        let t342 = t66 * t341;
        let t348 = 3.0 * t181 * t341 + 3.0 * t184 * t341;
        let t351 = piecewise3(t54, 6.0 * t61 * t333 + 3.0 * t336 - 12.0 * t338, 6.0 * t173 * t342 + t180 * t348);
        let t352 = t44 * t351;
        let t354 = t326 * t82;
        let t356 = t85 * t351;
        let t358 = t77 * t351;
        let t362 = 35.0 / 2.0 * t356 - 15.0 / 2.0 * t358;
        let t365 = t326 * t88;
        let t367 = t78 * t351;
        let t371 = 15.0 / 2.0 * t367 - 3.0 / 2.0 * t351;
        let t374 = t129 * t351;
        let t376 = t119 * t351;
        let t382 = t99 * t326;
        let t387 = 15.0 / 2.0 * t382 - t316 / 8.0 + t324 / 96.0;
        let t390 = t387 * t77;
        let t397 = 0.0318024096 * t327 + 0.0453837246 * t329 + 0.0453837246 * t352 - 1.00478906e-07 * t354 + 0.002476783505 * t356 + 0.043536371955 * t358 - 1.00478906e-07 * t44 * t362 - 0.00608338264 * t365 + 0.01055956065 * t367 - 0.00608338264 * t44 * t371 - 0.0222650139 * t374 + 0.0192374554 * t376 + 2.01895739e-07 * t129 * t362 - 6.57949254e-07 * t129 * t371 - 0.150848736 * t382 + 1.33707403e-07 * t387 * t92 + 0.0192374554 * t390 + 0.01214700985 * t316 + 3.97324768e-09 * t387 * t82 + 3.97324768e-09 * t119 * t362;
        let t402 = t116 * t326;
        let t404 = t44 * t326;
        let t408 = 35.0 / 2.0 * t402 - 15.0 / 2.0 * t404;
        let t437 = t408 * t77;
        let t439 = t106 * t351;
        let t442 = -0.01565454237 * t404 * t92 - 0.01565454237 * t136 * t351 - 0.0667950417 * t404 * t77 + 0.0954072288 * t95 * t351 - 1.502248044e-06 * t114 * t351 + 4.01122209e-07 * t126 * t351 + 5.74317889e-08 * t106 * t371 - 5.00749348e-07 * t408 * t92 + 9.19317034e-07 * t437 + 9.19317034e-07 * t439 - 0.0010122508208333333 * t324;
        let t444 = t397 - 5.49909413e-08 * t387 * t88 - 5.49909413e-08 * t119 * t371 + 0.024747993725 * t402 - 0.170256507825 * t404 - 3.40722258e-09 * t408 * t82 - 3.40722258e-09 * t106 * t362 + 5.74317889e-08 * t408 * t88 - 0.06972770593 * t351 + 6.05687217e-07 * t404 * t82 - 1.973847762e-06 * t404 * t88 + t442;
        let t448 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t444);
        let tvsigma0 = 2.0 * rho[ip] * t448;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t452 = 5.0 / 9.0 * t29 * t47 * t26;
        let t453 = piecewise3(t55, t452, 0.0);
        let t456 = t63 * t453;
        let t458 = t169 * t453;
        let t461 = piecewise3(t55, 0.0, t452);
        let t462 = t66 * t461;
        let t468 = 3.0 * t181 * t461 + 3.0 * t184 * t461;
        let t471 = piecewise3(t54, 6.0 * t61 * t453 + 3.0 * t456 - 12.0 * t458, 6.0 * t173 * t462 + t180 * t468);
        let t472 = t85 * t471;
        let t474 = t77 * t471;
        let t476 = 35.0 / 2.0 * t472 - 15.0 / 2.0 * t474;
        let t479 = t78 * t471;
        let t482 = 15.0 / 2.0 * t479 - 3.0 / 2.0 * t471;
        let t487 = t106 * t471;
        let t495 = t119 * t471;
        let t503 = t129 * t471;
        let t511 = t44 * t471;
        let t517 = -3.40722258e-09 * t106 * t476 + 5.74317889e-08 * t106 * t482 - 1.502248044e-06 * t114 * t471 + 9.19317034e-07 * t487 + 3.97324768e-09 * t119 * t476 - 5.49909413e-08 * t119 * t482 + 4.01122209e-07 * t126 * t471 + 0.0192374554 * t495 + 2.01895739e-07 * t129 * t476 - 6.57949254e-07 * t129 * t482 - 0.01565454237 * t136 * t471 - 0.0222650139 * t503 - 1.00478906e-07 * t44 * t476 - 0.00608338264 * t44 * t482 + 0.0954072288 * t95 * t471 + 0.0453837246 * t511 + 0.002476783505 * t472 + 0.01055956065 * t479 + 0.043536371955 * t474 - 0.06972770593 * t471;
        let t521 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t517);
        let tvtau0 = 2.0 * rho[ip] * t521;
        vtau[ip] += tvtau0;
    }
}
