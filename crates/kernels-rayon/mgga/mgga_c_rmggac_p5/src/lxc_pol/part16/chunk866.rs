//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 866/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk866(t39698: f64, t39701: f64, t39785: f64, t39796: f64, t39800: f64, t39808: f64, t39840: f64, t39842: f64, t39873: f64, t39899: f64, t39926: f64, t39970: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43107 = 0.10909864661698136692e0_f64 * t39698;
    let t43108 = 0.47896966807455234256e0_f64 * t39701;
    let t43135 = 0.60975299583150056624e-3_f64 * t39785;
    let t43138 = 0.60975299583150056624e-3_f64 * t39796;
    let t43139 = 0.60975299583150056624e-3_f64 * t39800;
    let t43141 = 0.86737941314158990616e-4_f64 * t39808;
    let t43157 = 0.49658699875514145965e-4_f64 * t39840;
    let t43158 = 0.11918087970123395032e-3_f64 * t39842;
    let t43169 = 0.39726959900411316772e-4_f64 * t39873;
    let t43179 = 0.10909864661698136692e0_f64 * t39899;
    let t43190 = 0.39726959900411316772e-4_f64 * t39926;
    let t43204 = 0.39726959900411316772e-4_f64 * t39970;
    (t43107, t43108, t43135, t43138, t43139, t43141, t43157, t43158, t43169, t43179, t43190, t43204)
}
