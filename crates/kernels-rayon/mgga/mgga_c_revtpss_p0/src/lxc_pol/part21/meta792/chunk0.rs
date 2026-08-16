//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2857/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2857(t52013: f64, t52016: f64, t52020: f64, t52023: f64, t52025: f64, t52028: f64, t52031: f64, t52033: f64, t52035: f64, t52037: f64, t52039: f64, t52041: f64) -> f64 {
    let t52043 = -0.11038e0_f64 * t52013 + 0.49671e0_f64 * t52016 - 0.149013e1_f64 * t52020 + 0.58258125e1_f64 * t52023 - 0.1237865625e0_f64 * t52025 + 0.36230999999999999999e1_f64 * t52028 + 0.40256666666666666666e1_f64 * t52031 + 0.181155e1_f64 * t52033 + 0.80513333333333333334e0_f64 * t52035 - 0.26837777777777777778e0_f64 * t52037 - 0.12077e1_f64 * t52039 - 0.60385e0_f64 * t52041;
    t52043
}
