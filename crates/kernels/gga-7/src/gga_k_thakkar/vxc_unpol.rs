//! GGA_K_THAKKAR vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 36 shared lines across all orders.
//! Delta: 29 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_k_thakkar_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
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
        // --- vxc delta (this level) (29 lines) ---
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
    }
}
