//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1239/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1239<F: Float>(t45551: F, t473: F, t1243: F, t2149: F, t37885: F, t12627: F, t7635: F, t2155: F, t44126: F, t1892: F, t786: F, t25877: F) -> (F, F, F, F, F, F) {
    let t97377 = t45551 * t473;
    let t97397 = t2149 * t37885 * t1243;
    let t97475 = t12627 * t7635;
    let t97498 = t2155 * t44126;
    let t97699 = t786 * t1892;
    let t97700 = t97699 * t25877;
    (t97377, t97397, t97475, t97498, t97699, t97700)
}
