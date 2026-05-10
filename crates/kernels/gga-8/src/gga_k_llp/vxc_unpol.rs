//! GGA_K_LLP vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 40 shared lines across all orders.
//! Delta: 28 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_k_llp_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (40 lines) ---
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
        let t24 = param_beta * t4;
        let t26 = pow_1_3(1.0 / M_PI);
        let t27 = 1.0 / t26;
        let t28 = M_CBRT4;
        let t29 = t27 * t28;
        let t30 = t24 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t22 / t34;
        let t37 = param_gamma * param_beta;
        let t38 = f64::sqrt(sigma[ip]);
        let t39 = t37 * t38;
        let t41 = 1.0 / t21 / rho[ip];
        let t45 = f64::ln(t38 * t31 * t41 + f64::sqrt(pow_2(t38 * t31 * t41) + 1.0));
        let t46 = t31 * t41 * t45;
        let t48 = 1.0 + t39 * t46;
        let t49 = 1.0 / t48;
        let t50 = t36 * t49;
        let t54 = 1.0 + 2.0 / 9.0 * t30 * t33 * t50;
        let t58 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t54);
        let tzk0 = 2.0 * t58;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (28 lines) ---
        let t60 = t20 / t21;
        let t64 = t34 * rho[ip];
        let t66 = 1.0 / t22 / t64;
        let t67 = t66 * t49;
        let t71 = t48 * t48;
        let t72 = 1.0 / t71;
        let t73 = t36 * t72;
        let t75 = 1.0 / t21 / t34;
        let t77 = t31 * t75 * t45;
        let t79 = t37 * sigma[ip];
        let t80 = t32 * t66;
        let t82 = t33 * t36 + 1.0;
        let t83 = f64::sqrt(t82);
        let t84 = 1.0 / t83;
        let t85 = t80 * t84;
        let t88 = -4.0 / 3.0 * t39 * t77 - 4.0 / 3.0 * t79 * t85;
        let t93 = -16.0 / 27.0 * t30 * t33 * t67 - 2.0 / 9.0 * t30 * t33 * t73 * t88;
        let t98 = piecewise3(t2, 0.0, t7 * t60 * t54 / 10.0 + 3.0 / 20.0 * t7 * t23 * t93);
        let tvrho0 = 2.0 * rho[ip] * t98 + 2.0 * t58;
        vrho[ip] += tvrho0;
        let t101 = t24 * t27;
        let t102 = t28 * t32;
        let t106 = t37 / t38;
        let t108 = t32 * t36;
        let t109 = t108 * t84;
        let t112 = t106 * t46 / 2.0 + t37 * t109 / 2.0;
        let t117 = -2.0 / 9.0 * t30 * t33 * t73 * t112 + 2.0 / 9.0 * t101 * t102 * t50;
        let t121 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t117);
        let tvsigma0 = 2.0 * rho[ip] * t121;
        vsigma[ip] += tvsigma0;
    }
}
