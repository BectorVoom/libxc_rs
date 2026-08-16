//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3043/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3043(t43780: f64, t43782: f64, t43816: f64, t44320: f64, t50952: f64, t50954: f64, t63355: f64, t63359: f64, t63361: f64, t63365: f64, t63370: f64, t63374: f64) -> f64 {
    let t63376 = 0.761037037037037037e-2_f64 * t50952 + 0.4566222222222222222e-1_f64 * t50954 + t44320 + 0.76103703703703703703e-2_f64 * t43780 + 0.15220740740740740741e-1_f64 * t43782 - 0.35515061728395061728e-1_f64 * t43816 + 0.17123333333333333333e-1_f64 * t63355 - 0.2283111111111111111e-1_f64 * t63359 + 0.1522074074074074074e-1_f64 * t63361 + 0.68493333333333333332e-1_f64 * t63365 - 0.68493333333333333331e-1_f64 * t63370 + 0.19025925925925925925e-1_f64 * t63374;
    t63376
}
