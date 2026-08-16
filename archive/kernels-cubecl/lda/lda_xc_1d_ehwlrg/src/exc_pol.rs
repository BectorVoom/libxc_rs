//! LDA_XC_1D_EHWLRG exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_1d_ehwlrg.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;

/// LDA_XC_1D_EHWLRG exc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_xc_1d_ehwlrg_exc_pol(
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
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t3 = t1 * t1;
        let t5 = param_a2 * t1 + param_a3 * t3 + param_a1;
        let t6 = f64::powf(t1, param_alpha);
        let tzk0 = t5 * t6;
        zk[ip] += tzk0;
    }
}
