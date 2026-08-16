//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2267/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2267(t25191: f64, t7898: f64, t1937: f64, t49686: f64, t75667: f64, t13426: f64, t6993: f64, t101436: f64, t101439: f64, t101472: f64, t101476: f64, t101482: f64, t101485: f64, t101486: f64, t101515: f64, t101542: f64, t13514: f64, t1502: f64, t1911: f64, t2007: f64, t2322: f64, t25800: f64, t25835: f64, t27145: f64, t27830: f64, t28053: f64, t3813: f64, t569: f64, t651: f64, t670: f64, t7725: f64) -> f64 {
    let t101546 = 6.0_f64 * t7898 * t25191;
    let t101548 = 2.0_f64 * t49686 * t1937;
    let t101550 = 4.0_f64 * t75667 * t1937;
    let t101552 = 4.0_f64 * t13426 * t6993;
    let t101555 = t25835 * t1911 + t101436 + t101439 - 2.0_f64 * t651 * t2007 * t13514 - 4.0_f64 * t2322 * t28053 - 4.0_f64 * t651 * t27830 * t670 - t101472 + t101476 - 4.0_f64 * t2322 * t27145 - t101482 - t101485 - t101486 + (t101515 + t101542) * t569 + t101546 - t101548 - t101550 - t101552 - t1502 * t25800 - t7725 * t3813;
    t101555
}
