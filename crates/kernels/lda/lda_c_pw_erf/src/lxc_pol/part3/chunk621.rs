//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 621/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk621<F: Float>(t3619: F, t589: F, t25: F, t3579: F, t3581: F, t3583: F, t3591: F, t3595: F, t3600: F, t3601: F, t3606: F, t3611: F, t3615: F) -> (F, F) {
    let t3620 = t589 * t3619;
    let t3623 = -F::cast_from(0.022222222222222223_f64) * t3579 + F::cast_from(0.013333333333333334_f64) * t3581 + F::cast_from(0.0044444444444444444_f64) * t3583 - F::cast_from(0.002962962962962963_f64) * t25 * t3591 - F::cast_from(0.006666666666666667_f64) * t25 * t3595 - t3600 - F::cast_from(0.02666666666666667_f64) * t3601 + F::cast_from(0.013333333333333334_f64) * t25 * t3606 - F::cast_from(0.006666666666666667_f64) * t25 * t3611 - F::new(0.04) * t25 * t3615 + F::new(0.04) * t25 * t3620;
    (t3620, t3623)
}
