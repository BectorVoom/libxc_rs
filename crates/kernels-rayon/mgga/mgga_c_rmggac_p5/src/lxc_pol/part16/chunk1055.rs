//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1055/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1055(t30453: f64, t3351: f64, t3352: f64, t515: f64, t8571: f64, t8587: f64, t38530: f64, t8432: f64, t8437: f64, t26287: f64, t46441: f64, t26283: f64, t46444: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47961 = t3351 * t3352 * t515 * t30453;
    let t47963 = t8571 * t8587;
    let t47966 = t38530 * t8432;
    let t47968 = t38530 * t8437;
    let t47970 = t26287 * t46441;
    let t47972 = t26283 * t46444;
    (t47961, t47963, t47966, t47968, t47970, t47972)
}
