//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 760/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk760<F: Float>(t3439: F, t4493: F, t4499: F, t4504: F, t4511: F, t4512: F, t4514: F, t4520: F, t4525: F, t4526: F, t4527: F, t4528: F, t4529: F, t4530: F, t4531: F, t4532: F, t4533: F) -> (F,) {
    let t5804 = t4493 + t4499 - t4504 + t4511 - t4512 + t4514 + t3439 + t4520 - t4525 + t4526 + t4527 - t4528 - t4529 + t4530 - t4531 - t4532 - t4533;
    (t5804,)
}
