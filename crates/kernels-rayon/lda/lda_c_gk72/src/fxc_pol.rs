//! LDA_C_GK72 fxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_gk72.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};
use libxc_rkernel_math::piecewise::{piecewise5};

/// LDA_C_GK72 fxc -- polarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_c_gk72_fxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t10 = t6 / t8;
        let t11 = t4 * t10;
        let t12 = t11 / 4.0;
        let t13 = t12 < 0.7;
        let t14 = f64::ln(t12);
        let t21 = t12 < 10.0;
        let t24 = t1 * t1;
        let t26 = t24 / t3;
        let t30 = f64::sqrt(4.0);
        let t31 = f64::sqrt(t11);
        let t36 = t3 * t3;
        let t38 = t1 / t36;
        let t39 = t8 * t8;
        let t43 = t24 * t36;
        let t45 = t5 / t39;
        let t49 = 1.0 / t31 / t43 / t45 / 4.0;
        let tzk0 = piecewise5(t13, 0.0311 * t14 - 0.048 + 0.00225 * t4 * t10 * t14 - 0.00425 * t11, t21, -0.06156 + 0.01898 * t14, 0.146 * t26 * t5 * t8 + 5.3 * t30 / t31 / t11 - 0.49 * t38 * t6 * t39 - 6.4 * t30 * t49);
        zk[ip] += tzk0;
        let t53 = 1.0 / t7;
        let t56 = 1.0 / t8 / t7;
        let t57 = t6 * t56;
        let t67 = f64::powf(4.0, 1.0 / 6.0);
        let t68 = t67 * t49;
        let t69 = t4 * t56;
        let t77 = 1.0 / t31 / t2 / t53 / 48.0;
        let t78 = t67 * t77;
        let t82 = piecewise5(t13, -0.010366666666666666 * t53 - 0.00075 * t4 * t57 * t14 + 0.0006666666666666666 * t4 * t57, t21, -0.006326666666666667 * t53, 0.048666666666666664 * t26 * t45 + 10.6 * t68 * t69 - 0.32666666666666666 * t38 * t10 - 21.333333333333332 * t78 * t69);
        let tvrho0 = t7 * t82 + tzk0;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t85 = t7 * t7;
        let t86 = 1.0 / t85;
        let t89 = 1.0 / t8 / t85;
        let t90 = t6 * t89;
        let t100 = t5 / t39 / t7;
        let t103 = t67 * t67;
        let t104 = t103 * t103;
        let t105 = t104 * t67;
        let t106 = t105 * t77;
        let t108 = 1.0 / t39 / t85;
        let t109 = t43 * t108;
        let t112 = t4 * t89;
        let t122 = 1.0 / t31 / t1 / t3 / t2 / t57 / 48.0;
        let t123 = t105 * t122;
        let t129 = piecewise5(t13, 0.010366666666666666 * t86 + 0.001 * t4 * t90 * t14 - 0.0006388888888888889 * t4 * t90, t21, 0.006326666666666667 * t86, -0.03244444444444444 * t26 * t100 + 8.833333333333334 * t106 * t109 - 14.133333333333333 * t68 * t112 + 0.10888888888888888 * t38 * t57 - 24.88888888888889 * t123 * t109 + 28.444444444444443 * t78 * t112);
        let tv2rho20 = t7 * t129 + 2.0 * t82;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}
