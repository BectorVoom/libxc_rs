//! GGA_X_HCTH_A vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 35 shared lines across all orders.
//! Delta: 23 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_hcth_a_vxc_unpol(
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
        // --- vxc delta (this level) (23 lines) ---
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
    }
}
