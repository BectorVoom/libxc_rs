//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1326/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1326(t114385: f64, t1936: f64, t30143: f64, t7741: f64, t30004: f64, t7889: f64, t22589: f64, t94982: f64, t25826: f64, t75833: f64, t22628: f64, t6998: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t114387 = 2.0_f64 * t114385 * t1936;
    let t114389 = 6.0_f64 * t30143 * t7741;
    let t114391 = 6.0_f64 * t7889 * t30004;
    let t114394 = t94982 * t22589;
    let t114396 = t25826 * t75833;
    let t114398 = t6998 * t22628;
    (t114387, t114389, t114391, t114394, t114396, t114398)
}
