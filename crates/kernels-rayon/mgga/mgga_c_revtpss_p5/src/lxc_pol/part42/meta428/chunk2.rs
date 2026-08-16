//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1493/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1493(t116: f64, t31653: f64, t31027: f64, t31629: f64, t31636: f64, t31032: f64, t31643: f64, t117918: f64, t117920: f64, t117927: f64, t117936: f64, t117938: f64, t117940: f64, t117997: f64, t1513: f64, t2357: f64, t31439: f64, t31443: f64, t36308: f64, t36315: f64) -> (f64, f64) {
    let t118630 = t116 * t31653;
    let t118649 = t31027 * t31629;
    let t118651 = t31027 * t31636;
    let t118653 = t31032 * t31643;
    let t118655 = -5.0_f64 / 2.0_f64 * t36308 * t117997 * t31439 + 5.0_f64 / 9.0_f64 * t36315 * t2357 * t1513 * t31443 + t117918 - t117920 - 10.0_f64 / 9.0_f64 * t117927 - 110.0_f64 / 27.0_f64 * t117936 + 44.0_f64 / 9.0_f64 * t117938 + 110.0_f64 / 27.0_f64 * t117940 + 20.0_f64 / 9.0_f64 * t118649 - 2.0_f64 / 3.0_f64 * t118651 - 50.0_f64 / 27.0_f64 * t118653;
    (t118630, t118655)
}
