//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2220/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2220(t108710: f64, t1937: f64, t108117: f64, t108120: f64, t108129: f64, t108681: f64, t108685: f64, t108687: f64, t108691: f64, t108693: f64, t1453: f64, t1502: f64, t1519: f64, t2007: f64, t21881: f64, t21882: f64, t27830: f64, t28030: f64, t28050: f64, t29986: f64, t30150: f64, t4246: f64, t4248: f64, t4257: f64, t4293: f64, t651: f64, t670: f64, t6985: f64, t7883: f64, t97622: f64) -> f64 {
    let t108712 = 2.0_f64 * t108710 * t1937;
    let t108713 = -2.0_f64 * t2007 * t21881 * t651 - 2.0_f64 * t29986 * t651 * t670 - 4.0_f64 * t108120 * t1519 + t1453 * t30150 - 2.0_f64 * t1502 * t27830 - 4.0_f64 * t1519 * t97622 - 2.0_f64 * t21882 * t6985 - 4.0_f64 * t28030 * t4257 - 4.0_f64 * t28030 * t4293 - 4.0_f64 * t28050 * t4248 - 2.0_f64 * t4246 * t7883 - t108117 - t108129 + t108681 - t108685 + t108687 + t108691 + t108693 - t108712;
    t108713
}
