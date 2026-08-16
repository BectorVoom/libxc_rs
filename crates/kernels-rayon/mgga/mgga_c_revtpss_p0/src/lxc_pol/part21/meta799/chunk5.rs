//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2898/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2898(t52037: f64, t52013: f64, t52016: f64, t52020: f64, t52023: f64, t52025: f64, t52028: f64, t52031: f64, t52033: f64, t52039: f64, t52041: f64, t52597: f64) -> f64 {
    let t52598 = 0.45908888888888888888e0_f64 * t52037;
    let t52601 = -0.13892666666666666667e0_f64 * t52013 + 0.62517e0_f64 * t52016 - 0.187551e1_f64 * t52020 + 0.794188125e1_f64 * t52023 - 0.473371875e0_f64 * t52025 + 0.61977000000000000001e1_f64 * t52028 + 0.68863333333333333334e1_f64 * t52031 + 0.309885e1_f64 * t52033 + t52597 - t52598 - 0.20659e1_f64 * t52039 - 0.103295e1_f64 * t52041;
    t52601
}
