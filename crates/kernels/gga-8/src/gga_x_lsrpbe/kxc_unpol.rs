//! GGA_X_LSRPBE kxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 32 shared lines across all orders.
//! Delta: 37 lines unique to kxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_lsrpbe_kxc_unpol(
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
    param_alpha: f64,
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (32 lines) ---
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
        let t20 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = param_mu * t20 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t34 = 1.0 / param_kappa;
        let t39 = f64::exp(-t26 * t29 * t33 * t34 / 24.0);
        let t42 = param_kappa + 1.0;
        let t48 = f64::exp(-param_alpha * t20 * t25 * t29 * t33 / 24.0);
        let t51 = 1.0 + param_kappa * (1.0 - t39) - t42 * (1.0 - t48);
        let t55 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t51);
        let tzk0 = 2.0 * t55;
        zk[ip] += tzk0;
        // --- vxc delta (15 lines) ---
        let t57 = t17 / t31;
        let t61 = t30 * rho[ip];
        let t63 = 1.0 / t31 / t61;
        let t67 = t42 * param_alpha;
        let t68 = t20 * t25;
        let t69 = t67 * t68;
        let t70 = t63 * t48;
        let t74 = -t26 * t29 * t63 * t39 / 9.0 + t69 * t29 * t70 / 9.0;
        let t79 = piecewise3(t2, 0.0, -t6 * t57 * t51 / 8.0 - 3.0 / 8.0 * t6 * t19 * t74);
        let tvrho0 = 2.0 * rho[ip] * t79 + 2.0 * t55;
        vrho[ip] += tvrho0;
        let t85 = t67 * t20;
        let t86 = t25 * t28;
        let t91 = t26 * t28 * t33 * t39 / 24.0 - t85 * t86 * t33 * t48 / 24.0;
        let t95 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t91);
        let tvsigma0 = 2.0 * rho[ip] * t95;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (38 lines) ---
        let t100 = t17 / t31 / rho[ip];
        let t107 = t30 * t30;
        let t109 = 1.0 / t31 / t107;
        let t114 = param_mu * param_mu;
        let t115 = t20 * t20;
        let t116 = t114 * t115;
        let t118 = 1.0 / t23 / t22;
        let t119 = sigma[ip] * sigma[ip];
        let t121 = t116 * t118 * t119;
        let t124 = 1.0 / t18 / t107 / t61;
        let t125 = t27 * t124;
        let t126 = t34 * t39;
        let t127 = t125 * t126;
        let t130 = t109 * t48;
        let t134 = param_alpha * param_alpha;
        let t135 = t42 * t134;
        let t137 = t135 * t115 * t118;
        let t138 = t119 * t27;
        let t139 = t124 * t48;
        let t143 = 11.0 / 27.0 * t26 * t29 * t109 * t39 - 2.0 / 81.0 * t121 * t127 - 11.0 / 27.0 * t69 * t29 * t130 + 2.0 / 81.0 * t137 * t138 * t139;
        let t148 = piecewise3(t2, 0.0, t6 * t100 * t51 / 12.0 - t6 * t57 * t74 / 4.0 - 3.0 / 8.0 * t6 * t19 * t143);
        let tv2rho20 = 2.0 * rho[ip] * t148 + 4.0 * t79;
        v2rho2[ip] += tv2rho20;
        let t158 = t118 * t27;
        let t159 = t116 * t158;
        let t160 = t107 * t30;
        let t162 = 1.0 / t18 / t160;
        let t170 = t27 * t162;
        let t171 = sigma[ip] * t48;
        let t175 = -t26 * t28 * t63 * t39 / 9.0 + t159 * t162 * sigma[ip] * t126 / 108.0 + t85 * t86 * t70 / 9.0 - t137 * t170 * t171 / 108.0;
        let t180 = piecewise3(t2, 0.0, -t6 * t57 * t91 / 8.0 - 3.0 / 8.0 * t6 * t19 * t175);
        let tv2rhosigma0 = 2.0 * rho[ip] * t180 + 2.0 * t95;
        v2rhosigma[ip] += tv2rhosigma0;
        let t183 = t116 * t118;
        let t184 = t107 * rho[ip];
        let t186 = 1.0 / t18 / t184;
        let t190 = t135 * t115;
        let t195 = -t183 * t27 * t186 * t126 / 288.0 + t190 * t158 * t186 * t48 / 288.0;
        let t199 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t195);
        let tv2sigma20 = 2.0 * rho[ip] * t199;
        v2sigma2[ip] += tv2sigma20;
        // --- kxc delta (this level) (37 lines) ---
        let t202 = t17 * t33;
        let t213 = 1.0 / t31 / t184;
        let t218 = t107 * t107;
        let t220 = 1.0 / t18 / t218;
        let t221 = t27 * t220;
        let t226 = t22 * t22;
        let t227 = 1.0 / t226;
        let t228 = t114 * param_mu * t227;
        let t229 = t119 * sigma[ip];
        let t230 = t228 * t229;
        let t231 = t218 * t61;
        let t232 = 1.0 / t231;
        let t233 = param_kappa * param_kappa;
        let t234 = 1.0 / t233;
        let t239 = t213 * t48;
        let t248 = t42 * t134 * param_alpha;
        let t249 = t248 * t227;
        let t254 = -154.0 / 81.0 * t26 * t29 * t213 * t39 + 22.0 / 81.0 * t121 * t221 * t126 - 8.0 / 243.0 * t230 * t232 * t234 * t39 + 154.0 / 81.0 * t69 * t29 * t239 - 22.0 / 81.0 * t137 * t138 * t220 * t48 + 8.0 / 243.0 * t249 * t229 * t232 * t48;
        let t259 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t202 * t51 + t6 * t100 * t74 / 4.0 - 3.0 / 8.0 * t6 * t57 * t143 - 3.0 / 8.0 * t6 * t19 * t254);
        let tv3rho30 = 2.0 * rho[ip] * t259 + 6.0 * t148;
        v3rho3[ip] += tv3rho30;
        let t277 = t218 * t30;
        let t278 = 1.0 / t277;
        let t279 = t228 * t278;
        let t281 = t119 * t234 * t39;
        let t294 = 11.0 / 27.0 * t26 * t28 * t109 * t39 - t159 * t124 * sigma[ip] * t126 / 12.0 + t279 * t281 / 81.0 - 11.0 / 27.0 * t85 * t86 * t130 + t137 * t125 * t171 / 12.0 - t249 * t278 * t119 * t48 / 81.0;
        let t299 = piecewise3(t2, 0.0, t6 * t100 * t91 / 12.0 - t6 * t57 * t175 / 4.0 - 3.0 / 8.0 * t6 * t19 * t294);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t299 + 4.0 * t180;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t308 = t218 * rho[ip];
        let t309 = 1.0 / t308;
        let t312 = t234 * sigma[ip] * t39;
        let t323 = t183 * t170 * t126 / 54.0 - t228 * t309 * t312 / 216.0 - t190 * t158 * t162 * t48 / 54.0 + t249 * t309 * sigma[ip] * t48 / 216.0;
        let t328 = piecewise3(t2, 0.0, -t6 * t57 * t195 / 8.0 - 3.0 / 8.0 * t6 * t19 * t323);
        let tv3rhosigma20 = 2.0 * rho[ip] * t328 + 2.0 * t199;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t331 = 1.0 / t218;
        let t339 = -t248 * t227 * t331 * t48 / 576.0 + t228 * t331 * t234 * t39 / 576.0;
        let t343 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t339);
        let tv3sigma30 = 2.0 * rho[ip] * t343;
        v3sigma3[ip] += tv3sigma30;
    }
}
