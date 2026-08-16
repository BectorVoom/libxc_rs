//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3085/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3085(t43780: f64, t43782: f64, t43816: f64, t43820: f64, t50952: f64, t50954: f64, t63355: f64, t63359: f64, t63361: f64, t63365: f64, t63370: f64, t63374: f64) -> f64 {
    let t63980 = 4.0_f64 / 27.0_f64 * t50952 + 8.0_f64 / 9.0_f64 * t50954 + t43820 + 4.0_f64 / 27.0_f64 * t43780 + 8.0_f64 / 27.0_f64 * t43782 - 56.0_f64 / 81.0_f64 * t43816 + t63355 / 3.0_f64 - 4.0_f64 / 9.0_f64 * t63359 + 8.0_f64 / 27.0_f64 * t63361 + 4.0_f64 / 3.0_f64 * t63365 - 4.0_f64 / 3.0_f64 * t63370 + 10.0_f64 / 27.0_f64 * t63374;
    t63980
}
