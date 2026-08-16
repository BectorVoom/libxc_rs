//! LDA_XC_1D_EHWLRG kxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_1d_ehwlrg.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]


/// LDA_XC_1D_EHWLRG kxc -- polarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_xc_1d_ehwlrg_kxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
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
        let t6 = f64::powf(t1, param_alpha);
        let tzk0 = t5 * t6;
        zk[ip] += tzk0;
        let t7 = param_a3 * t1;
        let t9 = param_a2 + 2.0 * t7;
        let tvrho0 = t1 * t9 * t6 + t5 * t6 * param_alpha + tzk0;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t14 = t9 * t6;
        let t16 = 1.0 / t1;
        let t17 = param_alpha * t16;
        let t23 = param_alpha * param_alpha;
        let t24 = t23 * t16;
        let tv2rho20 = 2.0 * t14 * param_alpha + tzk0 * t17 + tzk0 * t24 + 2.0 * t7 * t6 + 2.0 * t14;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t26 = param_a3 * t6;
        let t30 = 1.0 / t3;
        let t31 = param_alpha * t30;
        let t37 = t23 * param_alpha;
        let t38 = t37 * t30;
        let tv3rho30 = 3.0 * t14 * t17 + 3.0 * t14 * t24 + 6.0 * t26 * param_alpha - tzk0 * t31 + tzk0 * t38 + 6.0 * t26;
        v3rho3[ip * 4] += tv3rho30;
        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}
