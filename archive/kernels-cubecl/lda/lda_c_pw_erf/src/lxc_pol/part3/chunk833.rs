//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 833/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk833<F: Float>(t4570: F, t4573: F, t4578: F, t4580: F, t4583: F, t4584: F, t4585: F, t4586: F, t4587: F, t4591: F, t4593: F, t4595: F, t4649: F, t4705: F, t4707: F, t4708: F, t4709: F) -> F {
    let t5839 = -t4570 - t4573 - t4578 - t4580 + t4583 - t4584 + t4585 + t4586 - t4587 + t4591 - t4593 - t4595 + t4649 + t4705 + t4707 + t4708 + t4709;
    t5839
}
