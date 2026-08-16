//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 977/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk977(t3395: f64, t3403: f64, t1155: f64, t1138: f64, t3351: f64, t1136: f64, t3359: f64, t11135: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t11150: f64, t11156: f64, t11161: f64, t11165: f64, t11170: f64, t11174: f64) -> (f64, f64, f64, f64) {
    let t11433 = t3395 * t3403;
    let t11434 = t11433 * t1155;
    let t11437 = t1138 * t3351;
    let t11441 = t3351 * t3359 * t1136;
    let t11444 = 0.53272592592592592592e-1_f64 * t11135;
    let t11455 = -t11444 + 0.2283111111111111111e-1_f64 * t11137 + 0.11415555555555555555e-1_f64 * t11139 - 0.34246666666666666665e-1_f64 * t11141 - 0.17123333333333333333e-1_f64 * t11143 + 0.19025925925925925925e-1_f64 * t11150 - 0.68493333333333333331e-1_f64 * t11156 - 0.34246666666666666665e-1_f64 * t11161 + 0.10274e0_f64 * t11165 + 0.10274e0_f64 * t11170 + 0.17123333333333333333e-1_f64 * t11174;
    (t11434, t11437, t11441, t11455)
}
