//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1089/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1089<F: Float>(t94698: F, t94701: F, t2453: F, t26053: F, t7289: F, t94600: F, t2028: F, t3999: F, t25875: F, t25894: F, t25877: F, t94382: F, t7246: F, t9692: F, t1955: F, t7282: F, t9656: F) -> (F, F, F, F, F, F, F, F) {
    let t94703 = 0.51727911450665971904e-3 * t94701 * t94698;
    let t94725 = t2453 * t26053;
    let t94761 = 0.39982213492741449076e-1 * t7289 * t94600;
    let t94762 = t2028 * t3999;
    let t94763 = t25875 * t94762;
    let t94768 = t25894 * t94762;
    let t94771 = t94382 * t25877;
    let t94784 = 0.30356481678079769392e-1 * t7246 * t9692;
    let t94823 = t1955 * t7282 * t9656;
    (t94703, t94725, t94761, t94763, t94768, t94771, t94784, t94823)
}
