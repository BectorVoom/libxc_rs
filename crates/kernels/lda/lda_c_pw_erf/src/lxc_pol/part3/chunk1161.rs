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
    let t13607 = F::new(0.019753086419753086) * t13562 + F::new(0.28444444444444444) * t13564 + F::new(0.02666666666666667) * t10178 + t10195 - F::new(0.8638) * t13568 + F::new(0.8638) * t13571 + F::new(0.47988888888888886) * t13574 - F::new(0.8638) * t13577 + F::new(1.2957) * t13580 - F::new(0.10666666666666667) * t13583 + F::new(0.023994444444444443) * t13585 + F::new(0.03999074074074074) * t13587 - F::new(0.5278777777777778) * t13589 - F::new(0.023994444444444443) * t13592 + F::new(0.14396666666666666) * t13595 - F::new(0.10664197530864197) * t13600 + F::new(0.23994444444444443) * t13603 - F::new(0.008888888888888889) * t10196 + F::new(0.05925925925925926) * t10202;
    (t13600, t13603, t13607)
}
