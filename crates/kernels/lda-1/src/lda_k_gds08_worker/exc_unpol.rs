//! LDA_K_GDS08_WORKER exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 9 shared lines across all orders.
//! Delta: 9 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

/// LDA_K_GDS08_WORKER exc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_k_gds08_worker_exc_unpol(
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
        // --- shared preamble (9 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = 1.0 <= zeta_threshold;
        let t4 = zeta_threshold - 1.0;
        let t6 = piecewise5(t3, t4, t3, -t4, 0.0);
        let t7 = 1.0 + t6;
        let t9 = f64::ln(t7 * rho[ip]);
        let t11 = t9 * t9;
        let t16 = piecewise3(t2, 0.0, t7 * (param_C * t11 + param_B * t9 + param_A) / 2.0);
        let tzk0 = 2.0 * t16;
        zk[ip] += tzk0;
    }
}
