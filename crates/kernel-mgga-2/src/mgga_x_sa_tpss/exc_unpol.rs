//! MGGA_X_SA_TPSS exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 88 shared lines across all orders.
//! Delta: 88 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_sa_tpss_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (88 lines) ---
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = f64::sqrt(5.0);
        let t22 = M_PI * t21;
        let t23 = M_CBRT2;
        let t24 = t23 * t23;
        let t25 = tau[ip] * t24;
        let t26 = t19 * t19;
        let t28 = 1.0 / t26 / rho[ip];
        let t30 = sigma[ip] * t24;
        let t31 = rho[ip] * rho[ip];
        let t33 = 1.0 / t26 / t31;
        let t34 = t30 * t33;
        let t36 = t25 * t28 - t34 / 8.0;
        let t37 = M_CBRT6;
        let t38 = t36 * t37;
        let t39 = M_PI * M_PI;
        let t40 = pow_1_3(t39);
        let t41 = t40 * t40;
        let t42 = 1.0 / t41;
        let t43 = t38 * t42;
        let t45 = 5.0 * t43 + 9.0;
        let t46 = f64::sqrt(t45);
        let t47 = 5.0 / 9.0 * t43;
        let t48 = t47 + 0.348e0;
        let t49 = f64::ln(t48);
        let t50 = 0.2413e1 + t49;
        let t51 = f64::sqrt(t50);
        let t52 = 1.0 / t51;
        let t53 = t46 * t52;
        let t54 = t22 * t53;
        let t56 = sigma[ip] * sigma[ip];
        let t57 = 1.0 / t31;
        let t58 = t56 * t57;
        let t59 = tau[ip] * tau[ip];
        let t60 = 1.0 / t59;
        let t61 = t58 * t60;
        let t63 = 1.0 + t61 / 64.0;
        let t64 = t63 * t63;
        let t65 = 1.0 / t64;
        let t66 = t60 * t65;
        let t70 = (10.0 / 81.0 + 0.2485875e-1 * t58 * t66) * t37;
        let t71 = t70 * t42;
        let t74 = t47 - 1.0;
        let t75 = t42 * t74;
        let t78 = 1.0 + 0.22222222222222222222e0 * t38 * t75;
        let t79 = f64::sqrt(t78);
        let t80 = 1.0 / t79;
        let t83 = t37 * t42;
        let t84 = t83 * t34;
        let t86 = 9.0 / 20.0 * t74 * t80 + t84 / 36.0;
        let t87 = t86 * t86;
        let t90 = t37 * t37;
        let t92 = 1.0 / t40 / t39;
        let t93 = t90 * t92;
        let t94 = t56 * t23;
        let t95 = t31 * t31;
        let t96 = t95 * rho[ip];
        let t98 = 1.0 / t19 / t96;
        let t100 = t93 * t94 * t98;
        let t102 = 162.0 * t61 + 100.0 * t100;
        let t103 = f64::sqrt(t102);
        let t108 = 1.0 / t46;
        let t110 = 1.0 / M_PI * t21 * t108 * t51;
        let t114 = t56 * sigma[ip];
        let t115 = t95 * t95;
        let t116 = 1.0 / t115;
        let t119 = t71 * t34 / 24.0 + 146.0 / 2025.0 * t87 - 73.0 / 97200.0 * t86 * t103 + 25.0 / 104976.0 * t110 * t100 + 0.17218861679299947194e-2 * t61 + 0.60132076742768935544e-5 * t114 * t116;
        let t121 = 1.0 + 0.51656585037899841583e-1 * t84;
        let t122 = t121 * t121;
        let t123 = 1.0 / t122;
        let t125 = 2.0 / 45.0 * t54 + t119 * t123;
        let t126 = 1.0 / t125;
        let t130 = 1.0 - 2.0 / 45.0 * t22 * t53 * t126;
        let t134 = 1.0 + 2.0 / 45.0 * t22 * t53 * t130;
        let t138 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t134);
        let tzk0 = 2.0 * t138;
        zk[ip] += tzk0;
    }
}
