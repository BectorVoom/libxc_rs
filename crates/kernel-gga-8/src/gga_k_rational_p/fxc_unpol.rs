//! GGA_K_RATIONAL_P fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 30 shared lines across all orders.
//! Delta: 33 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_rational_p_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_C2: f64,
    param_p: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (30 lines) ---
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
        let t24 = 1.0 / param_p;
        let t26 = M_CBRT6;
        let t28 = M_PI * M_PI;
        let t29 = pow_1_3(t28);
        let t30 = t29 * t29;
        let t31 = 1.0 / t30;
        let t32 = t31 * sigma[ip];
        let t33 = M_CBRT2;
        let t34 = t33 * t33;
        let t35 = rho[ip] * rho[ip];
        let t42 = 1.0 + param_C2 * t24 * t26 * t32 * t34 / t22 / t35 / 24.0;
        let t43 = f64::powf(t42, -param_p);
        let t47 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t20 * t22 * t43);
        let tzk0 = 2.0 * t47;
        zk[ip] += tzk0;
        // --- vxc delta (12 lines) ---
        let t53 = t35 * rho[ip];
        let t57 = t7 * t20 / t53 * t43;
        let t58 = param_C2 * t26;
        let t60 = sigma[ip] * t34;
        let t61 = 1.0 / t42;
        let t63 = t58 * t31 * t60 * t61;
        let t67 = piecewise3(t2, 0.0, t7 * t20 / t21 * t43 / 10.0 + t57 * t63 / 60.0);
        let tvrho0 = 2.0 * rho[ip] * t67 + 2.0 * t47;
        vrho[ip] += tvrho0;
        let t74 = t31 * t34;
        let t76 = t58 * t74 * t61;
        let t79 = piecewise3(t2, 0.0, -t7 * t20 / t35 * t43 * t76 / 160.0);
        let tvsigma0 = 2.0 * rho[ip] * t79;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (33 lines) ---
        let t88 = t35 * t35;
        let t92 = t7 * t20 / t88 * t43;
        let t95 = t88 * t35;
        let t97 = 1.0 / t22 / t95;
        let t100 = t7 * t20 * t97 * t43;
        let t101 = param_C2 * param_C2;
        let t102 = t26 * t26;
        let t103 = t101 * t102;
        let t105 = 1.0 / t29 / t28;
        let t106 = t103 * t105;
        let t107 = sigma[ip] * sigma[ip];
        let t109 = t42 * t42;
        let t110 = 1.0 / t109;
        let t112 = t106 * t107 * t33 * t110;
        let t115 = t7 * t20;
        let t118 = t115 * t97 * t43 * t101;
        let t119 = t102 * t105;
        let t122 = t33 * t110 * t24;
        let t123 = t119 * t107 * t122;
        let t127 = piecewise3(t2, 0.0, -t7 * t20 / t21 / rho[ip] * t43 / 30.0 - 7.0 / 180.0 * t92 * t63 + t100 * t112 / 270.0 + t118 * t123 / 270.0);
        let tv2rho20 = 2.0 * rho[ip] * t127 + 4.0 * t67;
        v2rho2[ip] += tv2rho20;
        let t132 = t88 * rho[ip];
        let t134 = 1.0 / t22 / t132;
        let t137 = t7 * t20 * t134 * t43;
        let t140 = t106 * sigma[ip] * t33 * t110;
        let t149 = t119 * t33 * t110 * t24 * sigma[ip];
        let t153 = piecewise3(t2, 0.0, t57 * t76 / 80.0 - t137 * t140 / 720.0 - t115 * t134 * t43 * t101 * t149 / 720.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t153 + 2.0 * t79;
        v2rhosigma[ip] += tv2rhosigma0;
        let t160 = t7 * t20 / t22 / t88 * t43;
        let t163 = t103 * t105 * t33 * t110;
        let t165 = t106 * t122;
        let t169 = piecewise3(t2, 0.0, t160 * t163 / 1920.0 + t160 * t165 / 1920.0);
        let tv2sigma20 = 2.0 * rho[ip] * t169;
        v2sigma2[ip] += tv2sigma20;
    }
}
