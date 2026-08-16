//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 244/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk244(t198: f64, t673: f64, t1193: f64, t209: f64, t476: f64, t1156: f64, t23: f64, t77: f64, t9: f64, t31: f64, t212: f64, t222: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1194 = t673 * t198;
    let t1195 = t1193 * t1194;
    let t1196 = t476 * t209;
    let t1205 = t23 * t1156;
    let t1223 = 1.0_f64 / t9 / t77;
    let t1224 = t31 * t1223;
    let t1227 = 0.21341877202031537856e0_f64 * t212 * t1224 * t222;
    (t1194, t1195, t1196, t1205, t1223, t1224, t1227)
}
