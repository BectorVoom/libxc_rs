//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1334/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1334(t1907: f64, t6922: f64, t28196: f64, t28197: f64, t29589: f64, t7898: f64, t30005: f64, t4248: f64, t651: f64, t6765: f64, t7741: f64, t1868: f64, t6781: f64) -> (f64, f64, f64, f64, f64) {
    let t114780 = t1907 * t6922;
    let t114783 = 6.0_f64 * t28196 * t28197 * t114780;
    let t114785 = 3.0_f64 * t7898 * t29589;
    let t114787 = 6.0_f64 * t4248 * t30005;
    let t114790 = 6.0_f64 * t651 * t6765 * t7741;
    let t114791 = t1868 * t6781;
    (t114783, t114785, t114787, t114790, t114791)
}
