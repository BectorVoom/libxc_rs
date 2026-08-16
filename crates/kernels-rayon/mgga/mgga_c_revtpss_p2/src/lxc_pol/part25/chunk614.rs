//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 614/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk614(t1169: f64, t3471: f64, t1159: f64, t426: f64, t434: f64, t3453: f64, t3356: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64, t448: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3472 = t3471 * t1169;
    let t3475 = t1159 * t1159;
    let t3476 = 1.0_f64 / t3475;
    let t3477 = t426 * t3476;
    let t3478 = t434 * t434;
    let t3479 = 1.0_f64 / t3478;
    let t3480 = t3453 * t3479;
    let t3483 = 0.12361111111111111111e-1_f64 * t3356;
    let t3488 = t3483 - 0.61805555555555555556e-2_f64 * t3358 - 0.61805555555555555555e-2_f64 * t3365 + 0.18541666666666666667e-1_f64 * t3370 + 0.92708333333333333333e-2_f64 * t3374;
    let t3489 = t3488 * t448;
    (t3472, t3475, t3476, t3477, t3478, t3479, t3480, t3488, t3489)
}
