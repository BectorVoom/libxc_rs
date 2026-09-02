//! LDA_XC_1D_EHWLRG vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_1d_ehwlrg.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;

#[allow(unused_variables, non_snake_case)]
pub fn lda_xc_1d_ehwlrg_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
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
        let t7 = param_a3 * t1;
        let t9 = param_a2 + 2.0 * t7;
        let tvrho0 = t1 * t9 * t6 + t5 * t6 * param_alpha + tzk0;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
    }
}
