//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3066/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3066(t43780: f64, t43782: f64, t43816: f64, t43942: f64, t50952: f64, t50954: f64, t63355: f64, t63359: f64, t63361: f64, t63365: f64, t63370: f64, t63374: f64) -> f64 {
    let t63692 = 0.41203703703703703704e-2_f64 * t50952 + 0.24722222222222222223e-1_f64 * t50954 + t43942 + 0.41203703703703703703e-2_f64 * t43780 + 0.82407407407407407406e-2_f64 * t43782 - 0.19228395061728395061e-1_f64 * t43816 + 0.92708333333333333333e-2_f64 * t63355 - 0.12361111111111111111e-1_f64 * t63359 + 0.82407407407407407409e-2_f64 * t63361 + 0.37083333333333333334e-1_f64 * t63365 - 0.37083333333333333333e-1_f64 * t63370 + 0.10300925925925925926e-1_f64 * t63374;
    t63692
}
