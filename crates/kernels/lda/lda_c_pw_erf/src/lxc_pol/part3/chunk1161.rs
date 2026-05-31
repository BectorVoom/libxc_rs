//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1161/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1161<F: Float>(t10102: F, t11: F, t13598: F, t12160: F, t1953: F, t3633: F, t10178: F, t10195: F, t10196: F, t10202: F, t13562: F, t13564: F, t13568: F, t13571: F, t13574: F, t13577: F, t13580: F, t13583: F, t13585: F, t13587: F, t13589: F, t13592: F, t13595: F) -> (F, F, F) {
    let t13600 = t11 * t10102 * t13598;
    let t13603 = t1953 * t3633 * t12160;
    let t13607 = F::cast_from(0.019753086419753086_f64) * t13562 + F::cast_from(0.28444444444444444_f64) * t13564 + F::cast_from(0.02666666666666667_f64) * t10178 + t10195 - F::cast_from(0.8638_f64) * t13568 + F::cast_from(0.8638_f64) * t13571 + F::cast_from(0.47988888888888886_f64) * t13574 - F::cast_from(0.8638_f64) * t13577 + F::cast_from(1.2957_f64) * t13580 - F::cast_from(0.10666666666666667_f64) * t13583 + F::cast_from(0.023994444444444443_f64) * t13585 + F::cast_from(0.03999074074074074_f64) * t13587 - F::cast_from(0.5278777777777778_f64) * t13589 - F::cast_from(0.023994444444444443_f64) * t13592 + F::cast_from(0.14396666666666666_f64) * t13595 - F::cast_from(0.10664197530864197_f64) * t13600 + F::cast_from(0.23994444444444443_f64) * t13603 - F::cast_from(0.008888888888888889_f64) * t10196 + F::cast_from(0.05925925925925926_f64) * t10202;
    (t13600, t13603, t13607)
}
