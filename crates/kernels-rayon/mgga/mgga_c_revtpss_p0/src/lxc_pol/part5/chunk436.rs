//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 436/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk436(t118: f64, t1310: f64, t1315: f64, t1453: f64, t508: f64, t511: f64, t569: f64, t649: f64, t651: f64, t671: f64, t3: f64, t571: f64) -> (f64, f64, f64) {
    let t1455 = -t118 * t1310 + t1315 * t569 + t1453 * t511 - t508 * t649 - 2.0_f64 * t651 * t671;
    let t1456 = t3 * t1455;
    let t1458 = t3 * t571;
    (t1455, t1456, t1458)
}
