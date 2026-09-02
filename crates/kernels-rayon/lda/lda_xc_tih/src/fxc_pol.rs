//! LDA_XC_TIH fxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_tih.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;

#[allow(unused_variables, non_snake_case)]
pub fn lda_xc_tih_fxc_pol(
    rho: &[f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..vrho.len() / 2 {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t4 = rmath::tanh(1.0953 + 0.0334789 * rho0 + 0.0334789 * rho1);
        let t9 = rmath::tanh(-0.414661 + 0.152399 * rho0 + 0.152399 * rho1);
        let t14 = rmath::tanh(-0.354691 + 0.0390837 * rho0 + 0.0390837 * rho1);
        let t19 = rmath::tanh(0.0748531 + 0.136598 * rho0 + 0.136598 * rho1);
        let t24 = rmath::tanh(-1.41063 + 0.00496577 * rho0 + 0.00496577 * rho1);
        let t29 = rmath::tanh(0.48315 + 4.02905 * rho0 + 4.02905 * rho1);
        let t34 = rmath::tanh(-0.420166 + 0.0104352 * rho0 + 0.0104352 * rho1);
        let t39 = rmath::tanh(1.47409 + 0.442455 * rho0 + 0.442455 * rho1);
        let tvrho0 = 0.625039 - 1.30351 * t4 - 1.37026 * t9 - 1.29598 * t14 + 1.04305 * t19 - 0.909651 * t24 - 0.991782 * t29 - 0.915745 * t34 - 1.95026 * t39;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t41 = t4 * t4;
        let t43 = t9 * t9;
        let t45 = t14 * t14;
        let t47 = t19 * t19;
        let t49 = t24 * t24;
        let t51 = t29 * t29;
        let t53 = t34 * t34;
        let t55 = t39 * t39;
        let tv2rho20 = -5.03355413957527 + 0.043640080939 * t41 + 0.20882625374 * t43 + 0.050651693526 * t45 - 0.1424785439 * t47 + 0.00451711764627 * t49 + 3.9959392671 * t51 + 0.009555982224 * t53 + 0.8629022883 * t55;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}
