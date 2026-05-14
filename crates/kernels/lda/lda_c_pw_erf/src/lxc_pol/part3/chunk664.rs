//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 664/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk664<F: Float>(t4496: F, t4501: F, t4488: F, t108: F, t210: F, t267: F) -> (F, F, F, F) {
    let t4502 = t4501 * t4496;
    let t4504 = 8.0 / 27.0 * t4488 * t4502;
    let t4505 = t210 * t108;
    let t4506 = t4505 * t267;
    (t4502, t4504, t4505, t4506)
}
