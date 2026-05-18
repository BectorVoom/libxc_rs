//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1152/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1152<F: Float>(t10474: F, t2168: F, t4588: F, t518: F, t525: F, t12881: F, t3899: F, t4929: F, t571: F, t2146: F, t3748: F, t3752: F) -> (F, F, F, F, F, F) {
    let t13486 = F::new(4.0) / F::new(5.0) * t10474 * t2168;
    let t13487 = t4588 * t518;
    let t13489 = F::new(8.0) / F::new(15.0) * t13487 * t525;
    let t13491 = F::new(4.0) / F::new(5.0) * t12881 * t2168;
    let t13493 = t571 * t3899 * t4929;
    let t13494 = F::new(8.0) / F::new(5.0) * t13493;
    let t13495 = t2146 * t3748;
    let t13496 = F::new(16.0) / F::new(45.0) * t13495;
    let t13498 = F::new(8.0) / F::new(15.0) * t2146 * t3752;
    (t13486, t13489, t13491, t13494, t13496, t13498)
}
