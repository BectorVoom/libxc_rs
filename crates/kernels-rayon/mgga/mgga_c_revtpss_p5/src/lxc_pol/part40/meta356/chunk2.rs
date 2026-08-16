//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1227/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1227(t14622: f64, t4401: f64, t2414: f64, t4311: f64, t10428: f64, t1522: f64, t10613: f64, t10592: f64, t10596: f64, t10604: f64, t10611: f64, t14442: f64, t14443: f64, t14444: f64, t14615: f64, t14618: f64, t14620: f64, t14621: f64, t9542: f64) -> (f64, f64, f64, f64, f64) {
    let t14624 = 12.0_f64 * t4401 * t14622;
    let t14626 = 4.0_f64 * t4311 * t2414;
    let t14628 = 4.0_f64 * t10428 * t1522;
    let t14629 = 8.0_f64 * t10613;
    let t14630 = t10592 + t14442 - t14443 - t10596 - t14444 - t10604 + t9542 + t14615 - t14618 + t14620 + t14621 + t14624 - t10611 + t14626 + t14628 + t14629;
    (t14624, t14626, t14628, t14629, t14630)
}
