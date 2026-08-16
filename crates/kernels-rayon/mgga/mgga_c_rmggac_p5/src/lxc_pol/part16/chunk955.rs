//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 955/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk955(t26346: f64, t8548: f64, t5055: f64, t8551: f64, t1971: f64, t2144: f64, t30283: f64, t3351: f64, t30360: f64, t2289: f64, t38351: f64, t38943: f64, t8571: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45920 = t26346 * t8548;
    let t45922 = t5055 * t8551;
    let t45926 = t3351 * t1971 * t2144 * t30283;
    let t45930 = t3351 * t1971 * t2144 * t30360;
    let t45932 = t38351 * t2289;
    let t45934 = t8571 * t38943;
    (t45920, t45922, t45926, t45930, t45932, t45934)
}
