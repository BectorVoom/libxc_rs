//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2405/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2405<F: Float>(t11858: F, t15688: F, t12077: F, t15905: F, t994: F, t11725: F, t828: F, t225: F, t42059: F, t1053: F, t11940: F, t11240: F, t11628: F, t42646: F) -> (F, F, F, F, F, F) {
    let t43082 = t11858 * t15688;
    let t43105 = t994 * t12077 * t15905;
    let t43131 = t828 * t11725;
    let t43154 = t42059 * t225;
    let t43161 = t11940 * t1053;
    let t43207 = t11240 * t11628 * t42646;
    (t43082, t43105, t43131, t43154, t43161, t43207)
}
