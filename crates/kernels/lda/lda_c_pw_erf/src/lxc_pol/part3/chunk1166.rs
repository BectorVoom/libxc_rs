//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1166/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1166<F: Float>(t12289: F, t1953: F, t557: F, t325: F, t4694: F, t4672: F, t4606: F, t4690: F, t3618: F, t817: F, t1349: F, t3609: F) -> (F, F, F, F, F, F, F) {
    let t13720 = t1953 * t557 * t12289;
    let t13722 = t325 * t4694;
    let t13724 = t325 * t4672;
    let t13726 = t4606 * t4690;
    let t13729 = t1953 * t557 * t3618;
    let t13731 = t1953 * t817;
    let t13734 = t1953 * t1349 * t3609;
    (t13720, t13722, t13724, t13726, t13729, t13731, t13734)
}
