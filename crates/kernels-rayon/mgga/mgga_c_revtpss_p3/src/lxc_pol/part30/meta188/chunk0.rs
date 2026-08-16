//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 933/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk933(t225: f64, t4028: f64, t4043: f64, t1412: f64, t73: f64, t3829: f64, t1394: f64, t3889: f64, t1392: f64, t1395: f64, t539: f64, t541: f64) -> (f64, f64, f64, f64) {
    let t4045 = (t4028 + t4043) * t225;
    let t4049 = t73 * t1412;
    let t4050 = t4049 * t3829;
    let t4053 = t1394 * t3889;
    let t4056 = 6.0_f64 * t1392 * t1395 - t4045 * t541 - 12.0_f64 * t4050 * t539 + 3.0_f64 * t4053 * t539;
    (t4045, t4050, t4053, t4056)
}
