//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1304/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1304<F: Float>(t13551: F, t13556: F, t13558: F, t13559: F, t13560: F, t13561: F, t13746: F, t13748: F, t13750: F, t13752: F, t13755: F, t13764: F, t13765: F) -> F {
    let t15100 = t13551 - t13556 - t13558 + t13559 - t13560 - t13561 - t13746 - t13748 - t13750 - t13752 + t13755 - t13764 + t13765;
    t15100
}
