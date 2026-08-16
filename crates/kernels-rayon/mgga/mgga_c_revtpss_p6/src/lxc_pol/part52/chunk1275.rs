//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1275/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1275(t34241: f64, t531: f64, t2014: f64, t7238: f64, t108120: f64, t128477: f64, t128917: f64, t128920: f64, t128930: f64, t128932: f64, t128933: f64, t1310: f64, t1911: f64, t2056: f64, t28030: f64, t28653: f64, t32660: f64, t34188: f64, t508: f64, t5787: f64, t7007: f64, t7367: f64, t8695: f64, t97622: f64) -> f64 {
    let t128934 = t531 * t34241;
    let t128937 = 3.0_f64 * t2014 * t128934 * t7238;
    let t128941 = -2.0_f64 * t108120 * t2056 - t128477 * t508 - t1310 * t34188 + t1911 * t32660 - 2.0_f64 * t2056 * t97622 - 2.0_f64 * t28030 * t7367 - 2.0_f64 * t28653 * t7007 + t5787 * t8695 - t128917 - t128920 - t128930 - t128932 - t128933 + t128937;
    t128941
}
