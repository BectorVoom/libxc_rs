//! LDA_K_GDS08_WORKER exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 30 shared lines across all orders.
//! Delta: 30 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

/// LDA_K_GDS08_WORKER exc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_k_gds08_worker_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        // --- shared preamble (30 lines) ---
        let t1 = rho0 <= dens_threshold;
        let t2 = rho0 - rho1;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t7 = 1.0 + t5 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = 1.0 - t5 <= zeta_threshold;
        let t11 = -t8;
        let t12 = piecewise5(t7, t8, t10, t11, t5);
        let t13 = 1.0 + t12;
        let t16 = 2.0 * rho0 * t4 <= zeta_threshold;
        let t19 = 2.0 * rho1 * t4 <= zeta_threshold;
        let t20 = piecewise5(t16, t8, t19, t11, t5);
        let t21 = 1.0 + t20;
        let t23 = f64::ln(t21 * t3);
        let t25 = t23 * t23;
        let t27 = t23 * param_B + t25 * param_C + param_A;
        let t30 = piecewise3(t1, 0.0, t13 * t27 / 2.0);
        let t31 = rho1 <= dens_threshold;
        let t32 = piecewise5(t10, t8, t7, t11, -t5);
        let t33 = 1.0 + t32;
        let t34 = -t2;
        let t36 = piecewise5(t19, t8, t16, t11, t34 * t4);
        let t37 = 1.0 + t36;
        let t39 = f64::ln(t37 * t3);
        let t41 = t39 * t39;
        let t43 = t39 * param_B + t41 * param_C + param_A;
        let t46 = piecewise3(t31, 0.0, t33 * t43 / 2.0);
        let tzk0 = t30 + t46;
        zk[ip] += tzk0;
    }
}
