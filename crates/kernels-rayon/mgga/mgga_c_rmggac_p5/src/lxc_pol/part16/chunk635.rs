//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 635/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk635(t515: f64, t9044: f64, t1971: f64, t7230: f64, t498: f64, t570: f64, t7231: f64, t3351: f64, t5144: f64, t3352: f64, t9008: f64, t903: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9045 = t515 * t9044;
    let t9046 = t1971 * t9045;
    let t9047 = t7230 * t9046;
    let t9049 = t570 * t498;
    let t9050 = t515 * t9049;
    let t9051 = t7231 * t9050;
    let t9052 = t3351 * t9051;
    let t9054 = t515 * t5144;
    let t9055 = t3352 * t9054;
    let t9056 = t3351 * t9055;
    let t9060 = t903 * t9008;
    (t9046, t9047, t9051, t9052, t9055, t9056, t9060)
}
