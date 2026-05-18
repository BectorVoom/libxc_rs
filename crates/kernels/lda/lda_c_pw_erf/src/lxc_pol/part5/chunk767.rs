//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 767/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk767<F: Float>(t3373: F, t4092: F, t4095: F, t4096: F, t4099: F, t4103: F, t4106: F, t4113: F, t5894: F, t5897: F, t5898: F, t5904: F, t5907: F, t5911: F) -> F {
    let t7057 = -F::new(0.3350512821420176) * t5894 + t5897 + F::new(0.3350512821420176) * t5898 - t3373 + F::new(2.657442045789236) * t5904 - F::new(0.10611888591559791) * t5907 - t5911 - F::new(0.0837628205355044) * t4092 - t4095 - F::new(0.1675256410710088) * t4096 - t4099 + F::new(0.1675256410710088) * t4103 + t4106 + t4113;
    t7057
}
