//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2572/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2572<F: Float>(t221: F, t4019: F, t47293: F, t9995: F, t9905: F, t9976: F, t9984: F, t3978: F, t9921: F, t3926: F, t9909: F, t3930: F, t9901: F) -> (F, F, F, F, F) {
    let t47296 = t47293 * t4019 * t221 * t9995;
    let t47298 = t9976 * t9905;
    let t47300 = t221 * t9984;
    let t47302 = t3978 * t9921 * t47300;
    let t47304 = t9909 * t3926;
    let t47306 = t3930 * t9901;
    (t47296, t47298, t47302, t47304, t47306)
}
