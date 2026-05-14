//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1036/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1036<F: Float>(t102420: F, t5722: F, t28780: F, t98041: F, t27899: F, t28845: F, t28894: F, t97802: F, t98380: F, t97700: F, t1364: F, t30248: F, t786: F, t108379: F, t7515: F, t30226: F, t689: F) -> (F, F, F, F, F, F, F, F, F) {
    let t109534 = t102420 * t5722;
    let t109536 = t98041 * t28780;
    let t109539 = t27899 * t28845;
    let t109553 = t97802 * t28894;
    let t109555 = t98380 * t28894;
    let t109567 = t97700 * t28780;
    let t109579 = t786 * t30248 * t1364;
    let t109609 = t108379 * t7515;
    let t109630 = t30226 * t689;
    (t109534, t109536, t109539, t109553, t109555, t109567, t109579, t109609, t109630)
}
