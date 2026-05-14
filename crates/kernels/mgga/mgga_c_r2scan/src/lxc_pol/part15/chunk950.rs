//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 950/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk950<F: Float>(t3347: F, t5086: F, t1064: F, t23040: F, t3348: F, t481: F, t3270: F, t10913: F, t498: F, t1561: F, t3261: F, t97: F, t1065: F, t1234: F, t105: F, t1550: F) -> (F, F, F, F, F, F, F) {
    let t37292 = t5086 * t3347;
    let t37299 = t23040 * t1064;
    let t37312 = t3348 * t481;
    let t37313 = t3270 * t37312;
    let t37318 = t498 * t10913;
    let t37327 = t97 * t3261 * t1561;
    let t37341 = t1065 * t1234;
    let t37342 = t3270 * t37341;
    let t37346 = t97 * t105 * t1550;
    (t37292, t37299, t37313, t37318, t37327, t37342, t37346)
}
