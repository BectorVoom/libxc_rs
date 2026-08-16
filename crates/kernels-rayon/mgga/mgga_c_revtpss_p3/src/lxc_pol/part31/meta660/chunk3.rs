//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2238/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2238(t2014: f64, t2034: f64, t73407: f64, t25082: f64, t30122: f64, t32113: f64, t1448: f64, t6781: f64, t28196: f64, t98495: f64, t1353: f64, t28197: f64) -> (f64, f64, f64, f64) {
    let t109092 = t2014 * t2034 * t73407;
    let t109095 = 6.0_f64 * t25082 * t32113 * t30122;
    let t109096 = t6781 * t1448;
    let t109099 = 6.0_f64 * t28196 * t98495 * t109096;
    let t109100 = t6781 * t1353;
    let t109103 = 6.0_f64 * t25082 * t28197 * t109100;
    (t109092, t109095, t109099, t109103)
}
