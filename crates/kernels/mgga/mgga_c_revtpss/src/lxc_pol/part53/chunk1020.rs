//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1020/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1020<F: Float>(t125614: F, t125673: F, t125712: F, t125763: F, t125816: F, t125857: F, t125894: F, t125932: F, t1450: F, t2014: F, t532: F, t1448: F, t7933: F, t28196: F, t28197: F, t651: F, t7002: F, t7883: F) -> (F, F, F) {
    let t125938 = t2014 * t532 * (t125614 + t125673 + t125712 + t125763 + t125816 + t125857 + t125894 + t125932) * t1450;
    let t125939 = t7933 * t1448;
    let t125942 = 4.0 * t28196 * t28197 * t125939;
    let t125945 = t651 * t7883 * t7002;
    (t125938, t125942, t125945)
}
