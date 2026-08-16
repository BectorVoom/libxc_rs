//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1113/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1113<F: Float>(t1459: F, t1980: F, t38892: F, t7458: F, t1967: F, t9543: F, t1988: F, t9560: F, t9538: F, t1089: F, t15995: F, t2288: F, t598: F) -> (F, F, F, F, F) {
    let t39226 = t1980 * t7458 * t1459 * t38892;
    let t39228 = t1967 * t9543;
    let t39230 = t1988 * t9560;
    let t39232 = t1967 * t9538;
    let t39236 = t598 * t1089 * t15995 * t2288;
    (t39226, t39228, t39230, t39232, t39236)
}
