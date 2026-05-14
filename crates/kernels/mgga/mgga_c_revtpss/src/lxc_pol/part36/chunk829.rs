//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 829/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk829<F: Float>(t1086: F, t4746: F, t3090: F, t15822: F, t3160: F, t1655: F, t697: F, t1011: F, t1678: F, t3057: F, t3286: F, t1647: F, t3298: F, t994: F, t3316: F, t15669: F, t378: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15925 = t4746 * t1086;
    let t15926 = t15925 * t3090;
    let t15932 = t15822 * t3160;
    let t16219 = t697 * t1655;
    let t16220 = t1011 * t16219;
    let t16284 = t3057 * t1678;
    let t16502 = t4746 * t3286;
    let t16509 = t1647 * t3298;
    let t16543 = t1086 * t1678;
    let t16544 = t994 * t16543;
    let t16584 = t1647 * t3316;
    let t16600 = t15669 * t378;
    (t15926, t15932, t16220, t16284, t16502, t16509, t16544, t16584, t16600)
}
