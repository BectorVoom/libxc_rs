//! LDA_XC_1D_EHWLRG exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 4 shared lines across all orders.
//! Delta: 4 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;

/// LDA_XC_1D_EHWLRG exc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_xc_1d_ehwlrg_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (4 lines) ---
        let t1 = rho[ip] * rho[ip];
        let t4 = param_a2 * rho[ip] + param_a3 * t1 + param_a1;
        let t5 = f64::powf(rho[ip], param_alpha);
        let tzk0 = t4 * t5;
        zk[ip] += tzk0;
    }
}
