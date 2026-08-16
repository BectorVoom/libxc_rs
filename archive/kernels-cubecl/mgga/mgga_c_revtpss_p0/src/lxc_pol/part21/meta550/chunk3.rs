//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2227/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2227<F: Float>(t16750: F, t482: F, t371: F, t372: F, t1803: F, t3666: F, t1208: F, t5215: F) -> (F, F, F, F) {
    let t17278 = t482 * t16750;
    let t17280 = t371 * t372 * t17278;
    let t17283 = t3666 * t1803;
    let t17288 = t5215 * t1208;
    (t17278, t17280, t17283, t17288)
}
