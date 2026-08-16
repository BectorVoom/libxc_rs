//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1501/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1501(t117845: f64, t117889: f64, t118039: f64, t118083: f64, t2204: f64, t5808: f64, t1921: f64, t8330: f64, t1913: f64, t8349: f64, t31512: f64, t571: f64) -> (f64, f64, f64, f64, f64) {
    let t118085 = t117845 + t117889 + t118039 + t118083;
    let t118089 = 2.0_f64 * t2204 * t5808;
    let t118091 = 2.0_f64 * t8330 * t1921;
    let t118094 = 2.0_f64 * t1913 * t8349;
    let t118099 = 2.0_f64 * t571 * t31512;
    (t118085, t118089, t118091, t118094, t118099)
}
