//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1142/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1142<F: Float>(t16140: F, t184: F, t1980: F, t199: F, t2405: F, t2023: F, t7007: F, t16159: F, t4763: F, t6277: F, t4753: F, t7680: F) -> (F, F, F, F, F, F) {
    let t21060 = F::new(64.0) / F::new(45.0) * t16140;
    let t21064 = F::new(4.0) / F::new(5.0) * t2405 * t1980 * t184 * t199;
    let t21066 = F::new(8.0) / F::new(15.0) * t7007 * t2023;
    let t21067 = F::new(32.0) / F::new(15.0) * t16159;
    let t21069 = F::new(8.0) / F::new(15.0) * t4763 * t6277;
    let t21071 = F::new(16.0) / F::new(15.0) * t4753 * t7680;
    (t21060, t21064, t21066, t21067, t21069, t21071)
}
