//! LDA_XC_1D_EHWLRG fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_1d_ehwlrg.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]


/// LDA_XC_1D_EHWLRG fxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_xc_1d_ehwlrg_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = rho[ip] * rho[ip];
        let t4 = param_a2 * rho[ip] + param_a3 * t1 + param_a1;
        let t5 = f64::powf(rho[ip], param_alpha);
        let tzk0 = t4 * t5;
        zk[ip] += tzk0;
        let t6 = rho[ip] * param_a3;
        let t8 = 2.0 * t6 + param_a2;
        let tvrho0 = rho[ip] * t8 * t5 + t4 * t5 * param_alpha + tzk0;
        vrho[ip] += tvrho0;
        let t13 = t8 * t5;
        let t15 = 1.0 / rho[ip];
        let t16 = param_alpha * t15;
        let t22 = param_alpha * param_alpha;
        let t23 = t22 * t15;
        let tv2rho20 = 2.0 * t13 * param_alpha + tzk0 * t16 + tzk0 * t23 + 2.0 * t6 * t5 + 2.0 * t13;
        v2rho2[ip] += tv2rho20;
    }
}
