//! GGA_K_PG fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 34 shared lines across all orders.
//! Delta: 23 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_pg_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_pg_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (34 lines) ---
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
        let t24 = M_CBRT6;
        let t25 = M_PI * M_PI;
        let t26 = pow_1_3(t25);
        let t27 = t26 * t26;
        let t28 = 1.0 / t27;
        let t29 = t24 * t28;
        let t30 = M_CBRT2;
        let t31 = t30 * t30;
        let t32 = sigma[ip] * t31;
        let t33 = rho[ip] * rho[ip];
        let t35 = 1.0 / t22 / t33;
        let t36 = t32 * t35;
        let t40 = param_pg_mu * t24 * t28;
        let t43 = f64::exp(-t40 * t36 / 24.0);
        let t44 = 5.0 / 72.0 * t29 * t36 + t43;
        let t48 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t44);
        let tzk0 = 2.0 * t48;
        zk[ip] += tzk0;
        // --- vxc delta (10 lines) ---
        let t50 = t20 / t21;
        let t54 = t33 * rho[ip];
        let t56 = 1.0 / t22 / t54;
        let t64 = -5.0 / 27.0 * t29 * t32 * t56 + t40 * t32 * t56 * t43 / 9.0;
        let t69 = piecewise3(t2, 0.0, t7 * t50 * t44 / 10.0 + 3.0 / 20.0 * t7 * t23 * t64);
        let tvrho0 = 2.0 * rho[ip] * t69 + 2.0 * t48;
        vrho[ip] += tvrho0;
        let t72 = t31 * t35;
        let t78 = 5.0 / 72.0 * t29 * t72 - t40 * t72 * t43 / 24.0;
        let t82 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t78);
        let tvsigma0 = 2.0 * rho[ip] * t82;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (23 lines) ---
        let t87 = t20 / t21 / rho[ip];
        let t94 = t33 * t33;
        let t96 = 1.0 / t22 / t94;
        let t104 = param_pg_mu * param_pg_mu;
        let t105 = t24 * t24;
        let t106 = t104 * t105;
        let t108 = 1.0 / t26 / t25;
        let t109 = t106 * t108;
        let t110 = sigma[ip] * sigma[ip];
        let t111 = t110 * t30;
        let t114 = 1.0 / t21 / t94 / t54;
        let t119 = 55.0 / 81.0 * t29 * t32 * t96 - 11.0 / 27.0 * t40 * t32 * t96 * t43 + 2.0 / 81.0 * t109 * t111 * t114 * t43;
        let t124 = piecewise3(t2, 0.0, -t7 * t87 * t44 / 30.0 + t7 * t50 * t64 / 5.0 + 3.0 / 20.0 * t7 * t23 * t119);
        let tv2rho20 = 2.0 * rho[ip] * t124 + 4.0 * t69;
        v2rho2[ip] += tv2rho20;
        let t130 = t31 * t56;
        let t136 = t94 * t33;
        let t140 = sigma[ip] * t43;
        let t144 = -5.0 / 27.0 * t29 * t130 + t40 * t130 * t43 / 9.0 - t109 * t30 / t21 / t136 * t140 / 108.0;
        let t149 = piecewise3(t2, 0.0, t7 * t50 * t78 / 10.0 + 3.0 / 20.0 * t7 * t23 * t144);
        let tv2rhosigma0 = 2.0 * rho[ip] * t149 + 2.0 * t82;
        v2rhosigma[ip] += tv2rhosigma0;
        let t156 = t106 * t108 * t30 * t43;
        let t159 = piecewise3(t2, 0.0, t7 * t20 * t96 * t156 / 1920.0);
        let tv2sigma20 = 2.0 * rho[ip] * t159;
        v2sigma2[ip] += tv2sigma20;
    }
}
