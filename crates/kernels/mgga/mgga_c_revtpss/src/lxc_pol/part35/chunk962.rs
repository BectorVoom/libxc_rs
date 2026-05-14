//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 962/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk962<F: Float>(t72: F, t8015: F, t686: F, t7058: F, t7064: F, t689: F, t8011: F, t25431: F, t25411: F, t786: F, t7998: F, t789: F, t1580: F, t7384: F, t213: F, t7997: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28359 = t8015 * t72;
    let t28360 = t28359 * t686;
    let t28361 = t7058 * t28360;
    let t28366 = t7064 * t28360;
    let t28368 = t8011 * t689;
    let t28369 = t25431 * t28368;
    let t28371 = t25411 * t28368;
    let t28373 = t786 * t7998;
    let t28374 = t28373 * t789;
    let t28390 = t7384 * t1580;
    let t28391 = t689 * t28390;
    let t28394 = t213 * t7997;
    (t28359, t28360, t28361, t28366, t28368, t28369, t28371, t28373, t28374, t28390, t28391, t28394)
}
