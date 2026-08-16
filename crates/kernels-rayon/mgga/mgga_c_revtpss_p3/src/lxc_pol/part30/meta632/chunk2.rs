//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2200/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2200(t114: f64, t101468: f64, t508: f64, t651: f64, t530: f64, t7933: f64, t2014: f64, t25865: f64, t1353: f64, t22496: f64, t28167: f64, t8717: f64, t25082: f64, t73394: f64) -> (f64, f64, f64, f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t101469 = piecewise3(t115, 0.0_f64, t101468);
    let t101472 = 2.0_f64 * t651 * t508 * t101469;
    let t101473 = t530 * t7933;
    let t101476 = 6.0_f64 * t2014 * t101473 * t25865;
    let t101479 = t22496 * t1353;
    let t101482 = 12.0_f64 * t28167 * t8717 * t101479;
    let t101485 = 6.0_f64 * t25082 * t8717 * t73394;
    (t101469, t101472, t101476, t101482, t101485)
}
