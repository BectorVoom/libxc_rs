//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 409/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk409<F: Float>(t12: F, t1489: F, t176: F, t166: F, t161: F, t1080: F, t1083: F, t44: F, t131: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t1490 = t1489 * t176;
    let t1491 = t166 * t1490;
    let t1493 = t161 * t1491 / F::new(30.0);
    let t1497 = piecewise3::<f64>(t13, F::new(0.0), F::new(2.0) * t12 * t1083 + F::new(2.0) * t1080);
    let t1498 = t1497 * t44;
    let t1499 = t1498 * t131;
    (t1490, t1491, t1493, t1498, t1499)
}
