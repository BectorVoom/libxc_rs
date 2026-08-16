//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2951/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2951<F: Float>(t15711: F, t3106: F, t15935: F, t372: F, t15936: F, t4786: F, t1469: F, t3151: F, t3162: F, t606: F, t15904: F, t245: F) -> (F, F, F, F, F) {
    let t53724 = t3106 * t15711;
    let t53728 = t372 * t15935;
    let t53729 = t15936 * t4786;
    let t53735 = t1469 * t3151 * t3162 * t606;
    let t53739 = t15904 * t245;
    (t53724, t53728, t53729, t53735, t53739)
}
