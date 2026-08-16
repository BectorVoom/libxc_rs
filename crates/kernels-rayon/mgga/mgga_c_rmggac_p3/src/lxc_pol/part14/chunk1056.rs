//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1056/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1056(t35554: f64, t8571: f64, t1970: f64, t1971: f64, t209: f64, t40427: f64, t515: f64, t275: f64, t9031: f64, t40884: f64, t7204: f64, t118: f64, t2281: f64, t498: f64, t7418: f64) -> (f64, f64, f64, f64, f64) {
    let t41897 = t8571 * t35554;
    let t41902 = t1970 * t1971 * t515 * t40427 * t209;
    let t41905 = 2.0_f64 * t275 * t9031;
    let t41906 = t7204 * t40884;
    let t41914 = t7418 * t118 * t2281 * t498;
    (t41897, t41902, t41905, t41906, t41914)
}
