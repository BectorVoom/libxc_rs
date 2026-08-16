//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1284/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1284(t100142: f64, t100145: f64, t100148: f64, t100174: f64, t101003: f64, t101012: f64, t101018: f64, t101028: f64, t18508: f64, t26679: f64, t26685: f64, t26748: f64, t27832: f64, t27958: f64, t28939: f64, t4947: f64, t7703: f64) -> f64 {
    let t101031 = -0.69505208333333333333e-3_f64 * t7703 * t101003 + 0.46336805555555555556e-3_f64 * t26748 * t28939 + 0.46336805555555555557e-3_f64 * t27832 * t27958 - 0.27636574074074074073e-2_f64 * t100142 + 0.61836467013888888889e-4_f64 * t26685 * t101012 + 0.18424382716049382715e-2_f64 * t100145 - 0.16581944444444444444e-1_f64 * t100148 + 0.12367293402777777778e-3_f64 * t26685 * t101018 + 0.46336805555555555556e-3_f64 * t7703 * t4947 * t26679 * t18508 - 0.33163888888888888888e-2_f64 * t100174 + 0.30918233506944444445e-4_f64 * t26685 * t101028;
    t101031
}
