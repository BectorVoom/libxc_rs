//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1499/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1499(t114: f64, t117971: f64, t118017: f64, t101522: f64, t1312: f64, t13426: f64, t18227: f64, t2199: f64, t2201: f64, t2322: f64, t27123: f64, t27126: f64, t28219: f64, t31164: f64, t31201: f64, t31401: f64, t31459: f64, t4151: f64, t4248: f64, t49686: f64, t508: f64, t651: f64, t75485: f64, t75667: f64, t7732: f64, t7889: f64, t8307: f64, t8321: f64, t8325: f64, t8327: f64, t8406: f64, t98484: f64, t98487: f64) -> (f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t118019 = piecewise3(t115, 0.0_f64, t117971 + t118017);
    let t118039 = -2.0_f64 * t118019 * t508 * t651 + 2.0_f64 * t1312 * t4151 * t8406 + 2.0_f64 * t101522 * t2201 + 4.0_f64 * t13426 * t8325 - 4.0_f64 * t18227 * t8307 - 2.0_f64 * t2199 * t75485 + 2.0_f64 * t2201 * t49686 + 4.0_f64 * t2201 * t75667 + 2.0_f64 * t2201 * t98484 + 4.0_f64 * t2201 * t98487 + 4.0_f64 * t2322 * t31401 + 4.0_f64 * t2322 * t31459 - 4.0_f64 * t27123 * t8321 + 4.0_f64 * t27123 * t8325 - 4.0_f64 * t27126 * t8321 + 4.0_f64 * t28219 * t8325 + 4.0_f64 * t28219 * t8327 + 2.0_f64 * t31164 * t7889 - 4.0_f64 * t31201 * t4248 - 4.0_f64 * t31201 * t7732;
    (t118019, t118039)
}
