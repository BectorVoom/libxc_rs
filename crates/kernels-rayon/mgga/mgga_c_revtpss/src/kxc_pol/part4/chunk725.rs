//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 725/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk725(t3670: f64, t480: f64, t3568: f64, t482: f64, t371: f64, t372: f64, t1236: f64, t127: f64, t1235: f64, t221: f64, t462: f64, t696: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3671 = t3670 * t480;
    let t3672 = t482 * t3568;
    let t3674 = t371 * t372 * t3672;
    let t3678 = t371 * t127 * t1236;
    let t3679 = t1235 * t3678;
    let t3682 = t221 * t696 * t462;
    (t3671, t3672, t3674, t3678, t3679, t3682)
}
