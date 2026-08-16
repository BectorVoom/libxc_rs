//! GGA_K_LGAP kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lgap.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_lgap_kxc_unpol(
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
    param_kappa: f64,
    param_mu_0: f64,
    param_mu_1: f64,
    param_mu_2: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5::<f64>(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3::<f64>(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3::<f64>(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3::<f64>(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3::<f64>(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t25 = M_CBRT6;
        let t26 = t25 * t25;
        let t28 = M_PI * M_PI;
        let t29 = pow_1_3::<f64>(t28);
        let t31 = param_mu_0 * t26 / t29;
        let t32 = f64::sqrt(sigma[ip]);
        let t33 = M_CBRT2;
        let t34 = t32 * t33;
        let t36 = 1.0 / t21 / rho[ip];
        let t41 = param_mu_1 * t25;
        let t42 = t29 * t29;
        let t43 = 1.0 / t42;
        let t44 = t41 * t43;
        let t45 = t33 * t33;
        let t46 = sigma[ip] * t45;
        let t47 = rho[ip] * rho[ip];
        let t49 = 1.0 / t22 / t47;
        let t55 = param_mu_2 / t28;
        let t56 = t32 * sigma[ip];
        let t57 = t47 * t47;
        let t58 = 1.0 / t57;
        let t63 = f64::exp(-t31 * t34 * t36 / 12.0 - t44 * t46 * t49 / 24.0 - t55 * t56 * t58 / 24.0);
        let t66 = 1.0 + param_kappa * (1.0 - t63);
        let t70 = piecewise3::<f64>(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t66);
        let tzk0 = 2.0 * t70;
        zk[ip] += tzk0;
        let t71 = 1.0 / t21;
        let t72 = t20 * t71;
        let t76 = t7 * t20;
        let t77 = t22 * param_kappa;
        let t79 = 1.0 / t21 / t47;
        let t83 = t47 * rho[ip];
        let t85 = 1.0 / t22 / t83;
        let t89 = t57 * rho[ip];
        let t90 = 1.0 / t89;
        let t94 = t31 * t34 * t79 / 9.0 + t44 * t46 * t85 / 9.0 + t55 * t56 * t90 / 6.0;
        let t95 = t94 * t63;
        let t100 = piecewise3::<f64>(t2, 0.0, t7 * t72 * t66 / 10.0 - 3.0 / 20.0 * t76 * t77 * t95);
        let tvrho0 = 2.0 * rho[ip] * t100 + 2.0 * t70;
        vrho[ip] += tvrho0;
        let t103 = 1.0 / t32;
        let t104 = t103 * t33;
        let t108 = t43 * t45;
        let t115 = -t31 * t104 * t36 / 24.0 - t41 * t108 * t49 / 24.0 - t55 * t32 * t58 / 16.0;
        let t116 = t115 * t63;
        let t120 = piecewise3::<f64>(t2, 0.0, -3.0 / 20.0 * t76 * t77 * t116);
        let tvsigma0 = 2.0 * rho[ip] * t120;
        vsigma[ip] += tvsigma0;
        let t123 = t20 * t36;
        let t127 = t71 * param_kappa;
        let t132 = 1.0 / t21 / t83;
        let t137 = 1.0 / t22 / t57;
        let t141 = t57 * t47;
        let t142 = 1.0 / t141;
        let t146 = -7.0 / 27.0 * t31 * t34 * t132 - 11.0 / 27.0 * t44 * t46 * t137 - 5.0 / 6.0 * t55 * t56 * t142;
        let t147 = t146 * t63;
        let t151 = t94 * t94;
        let t152 = t151 * t63;
        let t157 = piecewise3::<f64>(t2, 0.0, -t7 * t123 * t66 / 30.0 - t76 * t127 * t95 / 5.0 - 3.0 / 20.0 * t76 * t77 * t147 - 3.0 / 20.0 * t76 * t77 * t152);
        let tv2rho20 = 2.0 * rho[ip] * t157 + 4.0 * t100;
        v2rho2[ip] += tv2rho20;
        let t172 = t31 * t104 * t79 / 18.0 + t41 * t108 * t85 / 9.0 + t55 * t32 * t90 / 4.0;
        let t173 = t172 * t63;
        let t177 = t7 * t23;
        let t178 = param_kappa * t115;
        let t179 = t178 * t95;
        let t183 = piecewise3::<f64>(t2, 0.0, -t76 * t127 * t116 / 10.0 - 3.0 / 20.0 * t76 * t77 * t173 - 3.0 / 20.0 * t177 * t179);
        let tv2rhosigma0 = 2.0 * rho[ip] * t183 + 2.0 * t120;
        v2rhosigma[ip] += tv2rhosigma0;
        let t186 = 1.0 / t56;
        let t187 = t186 * t33;
        let t194 = t31 * t187 * t36 / 48.0 - t55 * t103 * t58 / 32.0;
        let t195 = t194 * t63;
        let t198 = t115 * t115;
        let t199 = t198 * t63;
        let t204 = piecewise3::<f64>(t2, 0.0, -3.0 / 20.0 * t76 * t77 * t195 - 3.0 / 20.0 * t76 * t77 * t199);
        let tv2sigma20 = 2.0 * rho[ip] * t204;
        v2sigma2[ip] += tv2sigma20;
        let t211 = t36 * param_kappa;
        let t222 = 1.0 / t21 / t57;
        let t227 = 1.0 / t22 / t89;
        let t232 = 1.0 / t57 / t83;
        let t236 = 70.0 / 81.0 * t31 * t34 * t222 + 154.0 / 81.0 * t44 * t46 * t227 + 5.0 * t55 * t56 * t232;
        let t237 = t236 * t63;
        let t241 = param_kappa * t146;
        let t242 = t241 * t95;
        let t246 = t151 * t94 * t63;
        let t251 = piecewise3::<f64>(t2, 0.0, 2.0 / 45.0 * t7 * t20 * t79 * t66 + t76 * t211 * t95 / 10.0 - 3.0 / 10.0 * t76 * t127 * t147 - 3.0 / 10.0 * t76 * t127 * t152 - 3.0 / 20.0 * t76 * t77 * t237 - 9.0 / 20.0 * t177 * t242 - 3.0 / 20.0 * t76 * t77 * t246);
        let tv3rho30 = 2.0 * rho[ip] * t251 + 6.0 * t157;
        v3rho3[ip] += tv3rho30;
        let t261 = t7 * t72;
        let t273 = -7.0 / 54.0 * t31 * t104 * t132 - 11.0 / 27.0 * t41 * t108 * t137 - 5.0 / 4.0 * t55 * t32 * t142;
        let t274 = t273 * t63;
        let t278 = param_kappa * t172;
        let t279 = t278 * t95;
        let t282 = t178 * t147;
        let t285 = t178 * t152;
        let t289 = piecewise3::<f64>(t2, 0.0, t76 * t211 * t116 / 30.0 - t76 * t127 * t173 / 5.0 - t261 * t179 / 5.0 - 3.0 / 20.0 * t76 * t77 * t274 - 3.0 / 10.0 * t177 * t279 - 3.0 / 20.0 * t177 * t282 - 3.0 / 20.0 * t177 * t285);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t289 + 4.0 * t183;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t301 = -t31 * t187 * t79 / 36.0 + t55 * t103 * t90 / 8.0;
        let t302 = t301 * t63;
        let t306 = param_kappa * t194;
        let t307 = t306 * t95;
        let t313 = t178 * t173;
        let t316 = param_kappa * t198;
        let t317 = t316 * t95;
        let t321 = piecewise3::<f64>(t2, 0.0, -t76 * t127 * t195 / 10.0 - 3.0 / 20.0 * t76 * t77 * t302 - 3.0 / 20.0 * t177 * t307 - t76 * t127 * t199 / 10.0 - 3.0 / 10.0 * t177 * t313 - 3.0 / 20.0 * t177 * t317);
        let tv3rhosigma20 = 2.0 * rho[ip] * t321 + 2.0 * t204;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t324 = sigma[ip] * sigma[ip];
        let t326 = 1.0 / t32 / t324;
        let t327 = t326 * t33;
        let t334 = -t31 * t327 * t36 / 32.0 + t55 * t186 * t58 / 64.0;
        let t335 = t334 * t63;
        let t339 = t306 * t116;
        let t342 = t198 * t115;
        let t343 = t342 * t63;
        let t348 = piecewise3::<f64>(t2, 0.0, -3.0 / 20.0 * t76 * t77 * t335 - 9.0 / 20.0 * t177 * t339 - 3.0 / 20.0 * t76 * t77 * t343);
        let tv3sigma30 = 2.0 * rho[ip] * t348;
        v3sigma3[ip] += tv3sigma30;
    }
}
