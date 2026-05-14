//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1071/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1071<F: Float>(t28043: F, t7586: F, t28056: F, t125556: F, t125558: F, t125562: F, t125566: F, t27060: F, t27145: F, t28053: F, t29427: F, t29432: F, t32825: F, t32869: F, t4248: F, t4293: F, t7007: F, t7746: F) -> (F,) {
    let t129395 = t7586 * t28043;
    let t129407 = t7586 * t28056;
    let t129411 = -2.0 * t27060 * t7746 - 2.0 * t27145 * t7586 - 2.0 * t28053 * t7586 - 2.0 * t29427 * t7007 - 2.0 * t29432 * t7746 - 2.0 * t32825 * t4293 - 2.0 * t32869 * t4248 - 2.0 * t125556 + t125558 - t125562 + t125566 - 2.0 * t129395 - 2.0 * t129407;
    (t129411,)
}
