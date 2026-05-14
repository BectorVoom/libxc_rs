//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 770/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk770<F: Float>(t166: F, t117: F, t3033: F, t130: F, t972: F, t1164: F, t3266: F, t182: F, t851: F, t1015: F, t173: F, t157: F, t406: F, t879: F, t186: F, t3873: F) -> (F, F, F, F, F, F, F, F) {
    let t13461 = t166 * t166;
    let t13462 = 1.0 / t13461;
    let t13483 = 1.0 / t3033 / t117;
    let t13716 = t130 * t972;
    let t13889 = t1164 * t3266;
    let t14046 = t851 * t182;
    let t14423 = 1.0 / t1015 / t173;
    let t14575 = t879 * t406 * t157;
    let t14651 = 1.0 / t3873 / t186;
    (t13462, t13483, t13716, t13889, t14046, t14423, t14575, t14651)
}
