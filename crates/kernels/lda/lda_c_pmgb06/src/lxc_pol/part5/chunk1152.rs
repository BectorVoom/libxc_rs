//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1152/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1152<F: Float>(t161: F, t16595: F, t166: F, t851: F, t479: F, t7465: F, t2108: F, t2592: F, t486: F, t7443: F, t13182: F, t2469: F, t493: F) -> (F, F, F, F, F) {
    let t20843 = t161 * t166 * t16595 * t851 / F::new(10.0);
    let t20845 = t7465 * t479 / F::new(30.0);
    let t20847 = t2592 * t2108 / F::new(10.0);
    let t20849 = t486 * t7443 / F::new(10.0);
    let t20852 = t493 * t13182 * t2469 / F::new(9.0);
    (t20843, t20845, t20847, t20849, t20852)
}
