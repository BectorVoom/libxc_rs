//! LDA_XC_1D_EHWLRG lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_1d_ehwlrg.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]


#[allow(unused_variables, non_snake_case)]
pub fn lda_xc_1d_ehwlrg_lxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
    param_a2: f64,
    param_a3: f64,
    param_a1: f64,
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
        let t25 = param_a3 * t5;
        let t29 = 1.0 / t1;
        let t30 = param_alpha * t29;
        let t36 = t22 * param_alpha;
        let t37 = t36 * t29;
        let tv3rho30 = 3.0 * t13 * t16 + 3.0 * t13 * t23 + 6.0 * t25 * param_alpha - tzk0 * t30 + tzk0 * t37 + 6.0 * t25;
        v3rho3[ip] += tv3rho30;
        let t44 = 1.0 / t1 / rho[ip];
        let t54 = t22 * t22;
        let tv4rho40 = -tzk0 * t22 * t44 - 2.0 * tzk0 * t36 * t44 + tzk0 * t54 * t44 + 2.0 * tzk0 * param_alpha * t44 - 4.0 * t13 * t30 + 4.0 * t13 * t37 + 12.0 * t25 * t16 + 12.0 * t25 * t23;
        v4rho4[ip] += tv4rho40;
    }
}
