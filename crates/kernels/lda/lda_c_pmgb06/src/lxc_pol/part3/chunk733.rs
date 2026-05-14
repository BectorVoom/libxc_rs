//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 733/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk733<F: Float>(t4618: F, t4736: F, t4789: F, t4833: F, t4971: F, t5057: F, t5098: F, t5132: F, t5156: F, t5192: F, t5241: F, t5298: F, t5340: F, t5389: F, t5457: F, t5504: F) -> (F,) {
    let t5508 = t4618 + t4736 + t4789 + t4833 + t4971 + t5057 + t5098 + t5132 + t5156 + t5192 + t5241 + t5298 + t5340 + t5389 + t5457 + t5504;
    (t5508,)
}
