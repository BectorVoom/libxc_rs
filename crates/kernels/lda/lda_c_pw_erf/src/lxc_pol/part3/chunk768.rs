//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 768/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk768<F: Float>(t4899: F, t4903: F, t4905: F, t4909: F, t4915: F, t4917: F, t4919: F, t4921: F, t4923: F, t4925: F, t4927: F, t4932: F, t4935: F, t4940: F, t4945: F, t4948: F, t4950: F) -> (F,) {
    let t5857 = -t4899 + t4903 + t4905 - t4909 - t4915 + t4917 + t4919 - t4921 - t4923 - t4925 - t4927 - t4932 - t4935 - t4940 + t4945 - t4948 - t4950;
    (t5857,)
}
