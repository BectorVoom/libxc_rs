//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 814/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk814<F: Float>(t1530: F, t7336: F, t174: F, t30423: F, t3126: F, t3157: F, t7323: F, t577: F, t7851: F, t339: F, t1165: F, t30327: F, t3355: F, t604: F, t1035: F, t1966: F) -> (F, F, F, F, F, F) {
    let t30698 = t1530 * t7336;
    let t30714 = t30423 * t7323 * t174 * t3157 * t3126;
    let t30715 = 0.12734375e-1 * t30714;
    let t30716 = t7851 * t577;
    let t30717 = t30716 * t339;
    let t30725 = t30327 * t1165 * t604 * t3355;
    let t30727 = t1035 * t1966;
    (t30698, t30715, t30716, t30717, t30725, t30727)
}
