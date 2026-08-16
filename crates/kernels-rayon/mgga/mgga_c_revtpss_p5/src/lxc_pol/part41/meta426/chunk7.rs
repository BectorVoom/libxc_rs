//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1493/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1493(t118276: f64, t118413: f64, t118456: f64, t118500: f64, t670: f64, t8362: f64, t116: f64, t31555: f64, t117758: f64, t1459: f64, t1518: f64, t1916: f64, t1918: f64, t2187: f64, t22559: f64, t22568: f64, t31340: f64, t31365: f64, t31610: f64, t31613: f64, t31616: f64, t35739: f64, t4292: f64, t572: f64, t573: f64, t5795: f64, t5805: f64, t6941: f64, t6948: f64, t8289: f64, t8299: f64, t8377: f64, t8383: f64, t8386: f64, param_d: f64) -> (f64, f64) {
    let t118502 = t118276 + t118413 + t118456 + t118500;
    let t118507 = t670 * t8362;
    let t118527 = t116 * t31555;
    let t118533 = 12.0_f64 * t117758 * t1518 * t572 + t118502 * t573 * param_d + 12.0_f64 * t118507 * t1518 * t572 + 6.0_f64 * t118527 * t572 * t670 + 12.0_f64 * t35739 * t4292 * t572 + 12.0_f64 * t1459 * t31610 + 6.0_f64 * t1459 * t31613 + 3.0_f64 * t1459 * t31616 + 12.0_f64 * t1916 * t31365 + 6.0_f64 * t1918 * t31340 + 12.0_f64 * t2187 * t22559 + 3.0_f64 * t2187 * t22568 + 12.0_f64 * t5795 * t8383 + 6.0_f64 * t5795 * t8386 + 6.0_f64 * t5805 * t8377 + 3.0_f64 * t6941 * t8299 + 3.0_f64 * t6948 * t8289;
    (t118502, t118533)
}
