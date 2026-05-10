//! GGA_X_HCTH_A kxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 35 shared lines across all orders.
//! Delta: 54 lines unique to kxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_hcth_a_kxc_unpol(
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
        // --- shared preamble (35 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = t3 * t3;
        let t22 = pow_1_3(1.0 / M_PI);
        let t25 = M_CBRT4;
        let t26 = t20 / t22 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t34 = f64::sqrt(sigma[ip]);
        let t35 = t34 * t27;
        let t37 = 1.0 / t18 / rho[ip];
        let t39 = f64::ln(t35 * t37 + f64::sqrt(pow_2(t35 * t37) + 1.0));
        let t40 = t37 * t39;
        let t43 = 1.0 + 0.252e-1 * t35 * t40;
        let t46 = t43 * t43;
        let t47 = 1.0 / t46;
        let t49 = -0.251173e1 / t43 + 0.37198333333333333333e1 * t47;
        let t54 = 0.109878e1 + 0.93333333333333333332e-3 * t26 * t29 * t33 * t49;
        let t58 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t54);
        let tzk0 = 2.0 * t58;
        zk[ip] += tzk0;
        // --- vxc delta (23 lines) ---
        let t60 = t17 / t31;
        let t64 = t30 * rho[ip];
        let t66 = 1.0 / t31 / t64;
        let t73 = 1.0 / t18 / t30 * t39;
        let t77 = t29 * t33 + 1.0;
        let t78 = f64::sqrt(t77);
        let t79 = 1.0 / t78;
        let t80 = t66 * t79;
        let t83 = -0.336e-1 * t35 * t73 - 0.336e-1 * t29 * t80;
        let t87 = 1.0 / t46 / t43;
        let t88 = t87 * t83;
        let t90 = 0.251173e1 * t47 * t83 - 0.74396666666666666666e1 * t88;
        let t95 = -0.24888888888888888889e-2 * t26 * t29 * t66 * t49 + 0.93333333333333333332e-3 * t26 * t29 * t33 * t90;
        let t100 = piecewise3(t2, 0.0, -t6 * t60 * t54 / 8.0 - 3.0 / 8.0 * t6 * t19 * t95);
        let tvrho0 = 2.0 * rho[ip] * t100 + 2.0 * t58;
        vrho[ip] += tvrho0;
        let t103 = t28 * t33;
        let t108 = 1.0 / t34 * t27;
        let t113 = 0.126e-1 * t108 * t40 + 0.126e-1 * t103 * t79;
        let t116 = t87 * t113;
        let t118 = 0.251173e1 * t47 * t113 - 0.74396666666666666666e1 * t116;
        let t123 = 0.93333333333333333332e-3 * t26 * t103 * t49 + 0.93333333333333333332e-3 * t26 * t29 * t33 * t118;
        let t127 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t123);
        let tvsigma0 = 2.0 * rho[ip] * t127;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (44 lines) ---
        let t132 = t17 / t31 / rho[ip];
        let t139 = t30 * t30;
        let t141 = 1.0 / t31 / t139;
        let t150 = t83 * t83;
        let t155 = 1.0 / t18 / t64 * t39;
        let t158 = t141 * t79;
        let t161 = sigma[ip] * sigma[ip];
        let t162 = t161 * t27;
        let t165 = 1.0 / t18 / t139 / t64;
        let t167 = 1.0 / t78 / t77;
        let t171 = 0.784e-1 * t35 * t155 + 0.168e0 * t29 * t158 - 0.896e-1 * t162 * t165 * t167;
        let t174 = t46 * t46;
        let t175 = 1.0 / t174;
        let t176 = t175 * t150;
        let t180 = -0.502346e1 * t87 * t150 + 0.251173e1 * t47 * t171 + 0.22319e2 * t176 - 0.74396666666666666666e1 * t87 * t171;
        let t185 = 0.9125925925925925926e-2 * t26 * t29 * t141 * t49 - 0.49777777777777777778e-2 * t26 * t29 * t66 * t90 + 0.93333333333333333332e-3 * t26 * t29 * t33 * t180;
        let t190 = piecewise3(t2, 0.0, t6 * t132 * t54 / 12.0 - t6 * t60 * t95 / 4.0 - 3.0 / 8.0 * t6 * t19 * t185);
        let tv2rho20 = 2.0 * rho[ip] * t190 + 4.0 * t100;
        v2rho2[ip] += tv2rho20;
        let t196 = t28 * t66;
        let t213 = t139 * t30;
        let t215 = 1.0 / t18 / t213;
        let t216 = t27 * t215;
        let t217 = t167 * sigma[ip];
        let t220 = -0.168e-1 * t108 * t73 - 0.504e-1 * t196 * t79 + 0.336e-1 * t216 * t217;
        let t223 = t175 * t113;
        let t226 = t87 * t220;
        let t228 = -0.502346e1 * t116 * t83 + 0.251173e1 * t47 * t220 + 0.22319e2 * t223 * t83 - 0.74396666666666666666e1 * t226;
        let t233 = -0.24888888888888888889e-2 * t26 * t196 * t49 + 0.93333333333333333332e-3 * t26 * t103 * t90 - 0.24888888888888888889e-2 * t26 * t29 * t66 * t118 + 0.93333333333333333332e-3 * t26 * t29 * t33 * t228;
        let t238 = piecewise3(t2, 0.0, -t6 * t60 * t123 / 8.0 - 3.0 / 8.0 * t6 * t19 * t233);
        let tv2rhosigma0 = 2.0 * rho[ip] * t238 + 2.0 * t127;
        v2rhosigma[ip] += tv2rhosigma0;
        let t244 = t113 * t113;
        let t249 = 1.0 / t34 / sigma[ip] * t27;
        let t252 = 1.0 / sigma[ip];
        let t253 = t252 * t28;
        let t254 = t33 * t79;
        let t257 = t139 * rho[ip];
        let t259 = 1.0 / t18 / t257;
        let t263 = -0.63e-2 * t249 * t40 + 0.63e-2 * t253 * t254 - 0.126e-1 * t27 * t259 * t167;
        let t266 = t175 * t244;
        let t268 = t87 * t263;
        let t270 = -0.502346e1 * t87 * t244 + 0.251173e1 * t47 * t263 + 0.22319e2 * t266 - 0.74396666666666666666e1 * t268;
        let t275 = 0.18666666666666666666e-2 * t26 * t103 * t118 + 0.93333333333333333332e-3 * t26 * t29 * t33 * t270;
        let t279 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t275);
        let tv2sigma20 = 2.0 * rho[ip] * t279;
        v2sigma2[ip] += tv2sigma20;
        // --- kxc delta (this level) (54 lines) ---
        let t282 = t17 * t33;
        let t293 = 1.0 / t31 / t257;
        let t306 = t150 * t83;
        let t313 = 1.0 / t18 / t139 * t39;
        let t319 = t139 * t139;
        let t321 = 1.0 / t18 / t319;
        let t325 = t161 * sigma[ip];
        let t326 = t319 * t64;
        let t327 = 1.0 / t326;
        let t329 = t77 * t77;
        let t331 = 1.0 / t78 / t329;
        let t334 = -0.26133333333333333333e0 * t35 * t313 - 0.88853333333333333333e0 * t29 * t293 * t79 + 0.11050666666666666667e1 * t162 * t321 * t167 - 0.7168e0 * t325 * t327 * t331;
        let t338 = 1.0 / t174 / t43;
        let t341 = t175 * t83;
        let t346 = 0.1507038e2 * t175 * t306 - 0.1507038e2 * t88 * t171 + 0.251173e1 * t47 * t334 - 0.89276e2 * t338 * t306 + 0.66957e2 * t341 * t171 - 0.74396666666666666666e1 * t87 * t334;
        let t351 = -0.42587654320987654321e-1 * t26 * t29 * t293 * t49 + 0.27377777777777777778e-1 * t26 * t29 * t141 * t90 - 0.74666666666666666667e-2 * t26 * t29 * t66 * t180 + 0.93333333333333333332e-3 * t26 * t29 * t33 * t346;
        let t356 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t282 * t54 + t6 * t132 * t95 / 4.0 - 3.0 / 8.0 * t6 * t60 * t185 - 3.0 / 8.0 * t6 * t19 * t351);
        let tv3rho30 = 2.0 * rho[ip] * t356 + 6.0 * t190;
        v3rho3[ip] += tv3rho30;
        let t366 = t28 * t141;
        let t394 = t27 * t165;
        let t397 = t319 * t30;
        let t399 = 1.0 / t397 * t331;
        let t402 = 0.392e-1 * t108 * t155 + 0.2072e0 * t366 * t79 - 0.3472e0 * t394 * t217 + 0.2688e0 * t399 * t161;
        let t405 = t338 * t113;
        let t408 = t175 * t220;
        let t413 = t87 * t402;
        let t415 = 0.1507038e2 * t223 * t150 - 0.1004692e2 * t226 * t83 - 0.502346e1 * t116 * t171 + 0.251173e1 * t47 * t402 - 0.89276e2 * t405 * t150 + 0.44638e2 * t408 * t83 + 0.22319e2 * t223 * t171 - 0.74396666666666666666e1 * t413;
        let t420 = 0.9125925925925925926e-2 * t26 * t366 * t49 - 0.49777777777777777778e-2 * t26 * t196 * t90 + 0.93333333333333333332e-3 * t26 * t103 * t180 + 0.9125925925925925926e-2 * t26 * t29 * t141 * t118 - 0.49777777777777777778e-2 * t26 * t29 * t66 * t228 + 0.93333333333333333332e-3 * t26 * t29 * t33 * t415;
        let t425 = piecewise3(t2, 0.0, t6 * t132 * t123 / 12.0 - t6 * t60 * t233 / 4.0 - 3.0 / 8.0 * t6 * t19 * t420);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t425 + 4.0 * t238;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t453 = t319 * rho[ip];
        let t455 = 1.0 / t453 * t331;
        let t458 = 0.84e-2 * t249 * t73 - 0.84e-2 * t253 * t80 + 0.84e-1 * t216 * t167 - 0.1008e0 * t455 * sigma[ip];
        let t461 = t338 * t244;
        let t466 = t175 * t263;
        let t469 = t87 * t458;
        let t471 = 0.1507038e2 * t266 * t83 - 0.1004692e2 * t116 * t220 - 0.502346e1 * t268 * t83 + 0.251173e1 * t47 * t458 - 0.89276e2 * t461 * t83 + 0.44638e2 * t223 * t220 + 0.22319e2 * t466 * t83 - 0.74396666666666666666e1 * t469;
        let t476 = -0.49777777777777777776e-2 * t26 * t196 * t118 + 0.18666666666666666666e-2 * t26 * t103 * t228 - 0.24888888888888888889e-2 * t26 * t29 * t66 * t270 + 0.93333333333333333332e-3 * t26 * t29 * t33 * t471;
        let t481 = piecewise3(t2, 0.0, -t6 * t60 * t275 / 8.0 - 3.0 / 8.0 * t6 * t19 * t476);
        let tv3rhosigma20 = 2.0 * rho[ip] * t481 + 2.0 * t279;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t487 = t244 * t113;
        let t494 = 1.0 / t34 / t161 * t27;
        let t497 = 1.0 / t161;
        let t498 = t497 * t28;
        let t501 = t252 * t27;
        let t502 = t259 * t167;
        let t505 = 1.0 / t319;
        let t508 = 0.945e-2 * t494 * t40 - 0.945e-2 * t498 * t254 - 0.63e-2 * t501 * t502 + 0.378e-1 * t505 * t331;
        let t511 = t338 * t487;
        let t515 = t87 * t508;
        let t517 = 0.1507038e2 * t175 * t487 - 0.1507038e2 * t116 * t263 + 0.251173e1 * t47 * t508 - 0.89276e2 * t511 + 0.66957e2 * t223 * t263 - 0.74396666666666666666e1 * t515;
        let t522 = 0.27999999999999999999e-2 * t26 * t103 * t270 + 0.93333333333333333332e-3 * t26 * t29 * t33 * t517;
        let t526 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t522);
        let tv3sigma30 = 2.0 * rho[ip] * t526;
        v3sigma3[ip] += tv3sigma30;
    }
}
