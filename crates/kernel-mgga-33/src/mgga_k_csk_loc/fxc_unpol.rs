//! MGGA_K_CSK_LOC fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 57 shared lines across all orders.
//! Delta: 112 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_k_csk_loc_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    param_csk_a: f64,
    param_csk_cp: f64,
    param_csk_cq: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (57 lines) ---
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = t4 * t4;
        let t6 = M_CBRTPI;
        let t8 = t5 * t6 * M_PI;
        let t9 = 1.0 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t12 = piecewise5(t9, t10, t9, -t10, 0.0);
        let t13 = 1.0 + t12;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t13 <= zeta_threshold, t16 * zeta_threshold, t19 * t13);
        let t22 = pow_1_3(rho[ip]);
        let t23 = t22 * t22;
        let t24 = t21 * t23;
        let t25 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t30 = t25 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t37 = t33 * t36;
        let t39 = 5.0 / 72.0 * t30 * t37;
        let t40 = param_csk_cp * t25;
        let t41 = t40 * t29;
        let t44 = param_csk_cq * t25;
        let t45 = t44 * t29;
        let t46 = lapl[ip] * t32;
        let t48 = 1.0 / t23 / rho[ip];
        let t52 = t41 * t37 / 24.0 + t45 * t46 * t48 / 24.0 - t39;
        let t54 = f64::ln(1.0 - f64::EPSILON);
        let t55 = 1.0 / param_csk_a;
        let t56 = f64::powf(-t54, -t55);
        let t57 = t52 < -t56;
        let t58 = f64::ln(f64::EPSILON);
        let t59 = f64::powf(-t58, -t55);
        let t60 = -t59 < t52;
        let t61 = piecewise3(t60, -t59, t52);
        let t62 = -t56 < t61;
        let t63 = piecewise3(t62, t61, -t56);
        let t64 = f64::abs(t63);
        let t65 = f64::powf(t64, param_csk_a);
        let t66 = 1.0 / t65;
        let t67 = f64::exp(-t66);
        let t68 = 1.0 - t67;
        let t69 = f64::powf(t68, t55);
        let t70 = piecewise5(t57, 0.0, t60, 1.0, t69);
        let t72 = t52 * t70 + t39 + 1.0;
        let t76 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t72);
        let tzk0 = 2.0 * t76;
        zk[ip] += tzk0;
        // --- vxc delta (37 lines) ---
        let t78 = t21 / t22;
        let t82 = t34 * rho[ip];
        let t84 = 1.0 / t23 / t82;
        let t85 = t33 * t84;
        let t87 = 5.0 / 27.0 * t30 * t85;
        let t93 = -t41 * t85 / 9.0 - 5.0 / 72.0 * t45 * t46 * t36 + t87;
        let t95 = t69 * t66;
        let t96 = piecewise3(t60, 0.0, t93);
        let t97 = piecewise3(t62, t96, 0.0);
        let t99 = f64::abs(t63) / t63;
        let t100 = 1.0 / t64;
        let t102 = 1.0 / t68;
        let t103 = t67 * t102;
        let t104 = t99 * t100 * t103;
        let t106 = piecewise5(t57, 0.0, t60, 0.0, -t95 * t97 * t104);
        let t108 = t52 * t106 + t93 * t70 - t87;
        let t113 = piecewise3(t3, 0.0, t8 * t78 * t72 / 10.0 + 3.0 / 20.0 * t8 * t24 * t108);
        let tvrho0 = 2.0 * rho[ip] * t113 + 2.0 * t76;
        vrho[ip] += tvrho0;
        let t116 = t32 * t36;
        let t118 = 5.0 / 72.0 * t30 * t116;
        let t119 = t29 * t32;
        let t120 = t119 * t36;
        let t123 = t40 * t120 / 24.0 - t118;
        let t125 = piecewise3(t60, 0.0, t123);
        let t126 = piecewise3(t62, t125, 0.0);
        let t129 = piecewise5(t57, 0.0, t60, 0.0, -t95 * t126 * t104);
        let t131 = t123 * t70 + t52 * t129 + t118;
        let t135 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t131);
        let tvsigma0 = 2.0 * rho[ip] * t135;
        vsigma[ip] += tvsigma0;
        let t137 = t32 * t48;
        let t144 = piecewise3(t60, 0.0, t44 * t119 * t48 / 24.0);
        let t145 = piecewise3(t62, t144, 0.0);
        let t148 = piecewise5(t57, 0.0, t60, 0.0, -t95 * t145 * t104);
        let t150 = t45 * t137 * t70 / 24.0 + t52 * t148;
        let t154 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t150);
        let tvlapl0 = 2.0 * rho[ip] * t154;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
        // --- fxc delta (this level) (112 lines) ---
        let t159 = t21 / t22 / rho[ip];
        let t166 = t34 * t34;
        let t168 = 1.0 / t23 / t166;
        let t169 = t33 * t168;
        let t171 = 55.0 / 81.0 * t30 * t169;
        let t177 = 11.0 / 27.0 * t41 * t169 + 5.0 / 27.0 * t45 * t46 * t84 - t171;
        let t181 = t65 * t65;
        let t183 = t69 / t181;
        let t184 = t97 * t97;
        let t186 = t99 * t99;
        let t187 = t64 * t64;
        let t188 = 1.0 / t187;
        let t189 = t186 * t188;
        let t190 = t67 * t67;
        let t191 = t68 * t68;
        let t192 = 1.0 / t191;
        let t193 = t190 * t192;
        let t194 = t189 * t193;
        let t196 = t184 * t186;
        let t198 = t188 * t67;
        let t199 = t102 * param_csk_a;
        let t200 = t198 * t199;
        let t202 = piecewise3(t60, 0.0, t177);
        let t203 = piecewise3(t62, t202, 0.0);
        let t206 = t95 * t184;
        let t207 = 0.0;
        let t209 = t207 * t100 * t103;
        let t210 = t206 * t209;
        let t211 = t189 * t103;
        let t213 = t183 * t196;
        let t215 = t188 * t190;
        let t216 = t192 * param_csk_a;
        let t217 = t215 * t216;
        let t220 = piecewise5(t57, 0.0, t60, 0.0, -t95 * t203 * t104 + t183 * t184 * t194 + t95 * t196 * t200 - t213 * t200 + t206 * t211 - t213 * t217 - t210);
        let t222 = 2.0 * t93 * t106 + t177 * t70 + t52 * t220 + t171;
        let t227 = piecewise3(t3, 0.0, -t8 * t159 * t72 / 30.0 + t8 * t78 * t108 / 5.0 + 3.0 / 20.0 * t8 * t24 * t222);
        let tv2rho20 = 2.0 * rho[ip] * t227 + 4.0 * t113;
        v2rho2[ip] += tv2rho20;
        let t233 = t32 * t84;
        let t235 = 5.0 / 27.0 * t30 * t233;
        let t236 = t119 * t84;
        let t239 = -t40 * t236 / 9.0 + t235;
        let t243 = t97 * t186;
        let t244 = t183 * t243;
        let t245 = t192 * t126;
        let t246 = t215 * t245;
        let t248 = t126 * t186;
        let t249 = t95 * t248;
        let t250 = t199 * t97;
        let t251 = t198 * t250;
        let t253 = piecewise3(t60, 0.0, t239);
        let t254 = piecewise3(t62, t253, 0.0);
        let t257 = t126 * t97;
        let t259 = t95 * t257 * t209;
        let t260 = t102 * t97;
        let t261 = t198 * t260;
        let t263 = t183 * t248;
        let t265 = t216 * t97;
        let t266 = t215 * t265;
        let t269 = piecewise5(t57, 0.0, t60, 0.0, -t95 * t254 * t104 + t244 * t246 + t249 * t251 + t249 * t261 - t263 * t251 - t263 * t266 - t259);
        let t271 = t123 * t106 + t93 * t129 + t239 * t70 + t52 * t269 - t235;
        let t276 = piecewise3(t3, 0.0, t8 * t78 * t131 / 10.0 + 3.0 / 20.0 * t8 * t24 * t271);
        let tv2rhosigma0 = 2.0 * rho[ip] * t276 + 2.0 * t135;
        v2rhosigma[ip] += tv2rhosigma0;
        let t289 = t192 * t145;
        let t290 = t215 * t289;
        let t292 = t145 * t186;
        let t293 = t95 * t292;
        let t297 = piecewise3(t60, 0.0, -5.0 / 72.0 * t44 * t120);
        let t298 = piecewise3(t62, t297, 0.0);
        let t301 = t145 * t97;
        let t303 = t95 * t301 * t209;
        let t305 = t183 * t292;
        let t309 = piecewise5(t57, 0.0, t60, 0.0, -t95 * t298 * t104 + t244 * t290 + t293 * t251 - t305 * t251 + t293 * t261 - t305 * t266 - t303);
        let t311 = -5.0 / 72.0 * t45 * t116 * t70 + t45 * t137 * t106 / 24.0 + t93 * t148 + t52 * t309;
        let t316 = piecewise3(t3, 0.0, t8 * t78 * t150 / 10.0 + 3.0 / 20.0 * t8 * t24 * t311);
        let tv2rholapl0 = 2.0 * rho[ip] * t316 + 2.0 * t154;
        v2rholapl[ip] += tv2rholapl0;
        let tv2rhotau0 = 0.0;
        v2rhotau[ip] += tv2rhotau0;
        let t321 = t126 * t126;
        let t324 = t321 * t186;
        let t327 = piecewise3(t60, 0.0, 0.0);
        let t328 = piecewise3(t62, t327, 0.0);
        let t330 = t95 * t328 * t104;
        let t331 = t95 * t321;
        let t332 = t331 * t209;
        let t334 = t183 * t324;
        let t338 = piecewise5(t57, 0.0, t60, 0.0, t183 * t321 * t194 + t95 * t324 * t200 - t334 * t200 + t331 * t211 - t334 * t217 - t330 - t332);
        let t340 = 2.0 * t123 * t129 + t52 * t338;
        let t344 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t340);
        let tv2sigma20 = 2.0 * rho[ip] * t344;
        v2sigma2[ip] += tv2sigma20;
        let t352 = t198 * t199 * t126;
        let t354 = t145 * t126;
        let t355 = t95 * t354;
        let t356 = t355 * t209;
        let t357 = t102 * t126;
        let t358 = t198 * t357;
        let t361 = t216 * t126;
        let t362 = t215 * t361;
        let t365 = piecewise5(t57, 0.0, t60, 0.0, t263 * t290 + t293 * t352 + t293 * t358 - t305 * t352 - t305 * t362 - t330 - t356);
        let t367 = t45 * t137 * t129 / 24.0 + t123 * t148 + t52 * t365;
        let t371 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t367);
        let tv2sigmalapl0 = 2.0 * rho[ip] * t371;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let t376 = t145 * t145;
        let t379 = t376 * t186;
        let t382 = t95 * t376;
        let t383 = t382 * t209;
        let t385 = t183 * t379;
        let t389 = piecewise5(t57, 0.0, t60, 0.0, t183 * t376 * t194 + t95 * t379 * t200 - t385 * t200 + t382 * t211 - t385 * t217 - t330 - t383);
        let t391 = t45 * t137 * t148 / 12.0 + t52 * t389;
        let t395 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t391);
        let tv2lapl20 = 2.0 * rho[ip] * t395;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
    }
}
