//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 264/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk264(t1212: f64, t209: f64, t469: f64, t6: f64, t1183: f64, t77: f64, t9: f64, t31: f64, t212: f64, t222: f64, t1189: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1215 = t469 * t6 * t1212 * t209;
    let t1219 = t469 * t1183 * t209;
    let t1223 = 1.0_f64 / t9 / t77;
    let t1224 = t31 * t1223;
    let t1227 = 0.21341877202031537856e0_f64 * t212 * t1224 * t222;
    let t1228 = t212 * t1189;
    (t1215, t1219, t1223, t1224, t1227, t1228)
}
