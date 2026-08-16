//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1406/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1406(t20735: f64, t21357: f64, t21393: f64, t21633: f64, t12587: f64, t6752: f64, t1298: f64, t1300: f64, t198: f64, t20571: f64, t20573: f64, t20576: f64, t20579: f64, t20582: f64, t20631: f64, t20633: f64, t20635: f64, t20637: f64, t20639: f64, t20643: f64, t20647: f64, t20650: f64, t20654: f64, t20692: f64, t20889: f64, t20894: f64, t20898: f64, t336: f64, t5023: f64) -> f64 {
    let t21635 = t20735 + t21357 + t21393 + t21633;
    let t21639 = t6752 * t12587;
    let t21643 = t1300 * t198 * t21635 * t336 - t1298 * t20692 * t5023 + 2.0_f64 * t1298 * t21639 * t5023 - t20571 + t20573 + t20576 - t20579 - t20582 + t20631 + t20633 + t20635 - t20637 + t20639 - t20643 + t20647 + t20650 + t20654 + t20889 - t20894 - t20898;
    t21643
}
