//! LDA_XC_TIH vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_vxc/lda_xc_tih.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]


/// LDA_XC_TIH vxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_xc_tih_vxc_unpol(
    rho: &[f64],
    vrho: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..vrho.len() {
        let t3 = f64::tanh(1.0953 + 0.0334789 * rho[ip]);
        let t7 = f64::tanh(-0.414661 + 0.152399 * rho[ip]);
        let t11 = f64::tanh(-0.354691 + 0.0390837 * rho[ip]);
        let t15 = f64::tanh(0.0748531 + 0.136598 * rho[ip]);
        let t19 = f64::tanh(-1.41063 + 0.00496577 * rho[ip]);
        let t23 = f64::tanh(0.48315 + 4.02905 * rho[ip]);
        let t27 = f64::tanh(-0.420166 + 0.0104352 * rho[ip]);
        let t31 = f64::tanh(1.47409 + 0.442455 * rho[ip]);
        let tvrho0 = 0.625039 - 1.30351 * t3 - 1.37026 * t7 - 1.29598 * t11 + 1.04305 * t15 - 0.909651 * t19 - 0.991782 * t23 - 0.915745 * t27 - 1.95026 * t31;
        vrho[ip] += tvrho0;
    }
}
