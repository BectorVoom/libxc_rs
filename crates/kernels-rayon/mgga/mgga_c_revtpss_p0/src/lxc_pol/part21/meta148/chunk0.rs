//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 945/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk945(t3453: f64, t3479: f64, t3356: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64) -> (f64, f64, f64) {
    let t3480 = t3453 * t3479;
    let t3483 = 0.12361111111111111111e-1_f64 * t3356;
    let t3488 = t3483 - 0.61805555555555555556e-2_f64 * t3358 - 0.61805555555555555555e-2_f64 * t3365 + 0.18541666666666666667e-1_f64 * t3370 + 0.92708333333333333333e-2_f64 * t3374;
    (t3480, t3483, t3488)
}
