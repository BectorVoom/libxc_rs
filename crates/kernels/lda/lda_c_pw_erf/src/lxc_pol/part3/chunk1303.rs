//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1303/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1303<F: Float>(t13512: F, t13516: F, t13518: F, t13524: F, t13528: F, t13531: F, t13534: F, t13537: F, t13539: F, t13541: F, t13543: F, t13545: F, t13547: F, t13549: F) -> F {
    let t15095 = -t13512 - t13516 - t13518 + t13524 - t13528 - t13531 - t13534 - t13537 - t13539 + t13541 + t13543 + t13545 - t13547 + t13549;
    t15095
}
