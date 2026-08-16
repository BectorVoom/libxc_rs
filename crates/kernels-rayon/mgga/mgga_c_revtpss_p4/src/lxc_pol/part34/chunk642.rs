//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 642/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk642(t1169: f64, t6502: f64, t3479: f64, t6486: f64, t3483: f64, t5044: f64, t6423: f64, t6427: f64, t6431: f64, t448: f64, t1756: f64) -> (f64, f64, f64, f64, f64) {
    let t6503 = t6502 * t1169;
    let t6506 = t6486 * t3479;
    let t6513 = t3483 - 0.61805555555555555556e-2_f64 * t5044 - 0.61805555555555555555e-2_f64 * t6423 + 0.18541666666666666667e-1_f64 * t6427 + 0.92708333333333333333e-2_f64 * t6431;
    let t6514 = t6513 * t448;
    let t6518 = t1756 * t1756;
    (t6503, t6506, t6513, t6514, t6518)
}
