//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1017/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1017<F: Float>(t11907: F, t11909: F, t4506: F, t219: F, t4048: F, t3589: F, t9234: F, t9244: F, t9246: F, t9251: F, t1506: F, t184: F, t494: F, t786: F) -> (F, F, F, F, F, F, F, F) {
    let t11912 = F::new(8.0) / F::new(3.0) * t4506 * t11907 * t11909;
    let t11913 = t4048 * t219;
    let t11914 = t11913 * t3589;
    let t11917 = F::new(32.0) / F::new(27.0) * t4506 * t11914 * t11909;
    let t11918 = F::new(8.0) / F::new(15.0) * t9234;
    let t11919 = F::new(8.0) / F::new(45.0) * t9244;
    let t11920 = F::new(16.0) / F::new(135.0) * t9246;
    let t11921 = F::new(4.0) / F::new(45.0) * t9251;
    let t11925 = F::new(4.0) / F::new(5.0) * t494 * t1506 * t184 * t786;
    (t11912, t11914, t11917, t11918, t11919, t11920, t11921, t11925)
}
