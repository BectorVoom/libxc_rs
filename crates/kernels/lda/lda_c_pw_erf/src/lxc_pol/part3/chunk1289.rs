//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1289/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1289<F: Float>(t12996: F, t12999: F, t13001: F, t13009: F, t13011: F, t13013: F, t13016: F, t13018: F, t13022: F, t13025: F, t13028: F, t13031: F, t13033: F) -> F {
    let t15065 = -t12996 + t12999 + t13001 + t13009 + t13011 - t13013 + t13016 - t13018 - t13022 - t13025 - t13028 - t13031 - t13033;
    t15065
}
