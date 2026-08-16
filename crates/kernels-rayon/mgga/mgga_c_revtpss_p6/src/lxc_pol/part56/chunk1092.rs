//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1092/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1092(t13648: f64, t2014: f64, t8595: f64, t33651: f64, t7312: f64, t28167: f64, t37956: f64, t5627: f64, t27833: f64, t8596: f64, t1353: f64, t7933: f64) -> (f64, f64, f64, f64, f64) {
    let t125525 = t2014 * t8595 * t13648;
    let t125531 = 2.0_f64 * t2014 * t7312 * t33651;
    let t125536 = 6.0_f64 * t28167 * t37956 * t5627;
    let t125558 = t27833 * t8596;
    let t125559 = t7933 * t1353;
    (t125525, t125531, t125536, t125558, t125559)
}
