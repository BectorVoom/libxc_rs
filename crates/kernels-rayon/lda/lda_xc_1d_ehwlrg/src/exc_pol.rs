//! LDA_XC_1D_EHWLRG exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_1d_ehwlrg.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;

#[allow(unused_variables, non_snake_case)]
pub fn lda_xc_1d_ehwlrg_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
    param_a2: f64,
    param_a3: f64,
    param_a1: f64,
    param_alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t3 = t1 * t1;
        let t5 = param_a2 * t1 + param_a3 * t3 + param_a1;
        let t6 = rmath::pow(t1, param_alpha);
        let tzk0 = t5 * t6;
        zk[ip] += tzk0;
    }
}
