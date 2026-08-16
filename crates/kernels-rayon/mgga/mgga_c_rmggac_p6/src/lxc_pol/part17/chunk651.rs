//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 651/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk651(t3351: f64, t9051: f64, t5144: f64, t515: f64, t3352: f64, t2028: f64, t2868: f64, t9008: f64, t903: f64, t1550: f64, t9000: f64, t1685: f64, t668: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9052 = t3351 * t9051;
    let t9054 = t515 * t5144;
    let t9055 = t3352 * t9054;
    let t9056 = t3351 * t9055;
    let t9058 = t2868 * t2028;
    let t9060 = t903 * t9008;
    let t9062 = t1550 * t9000;
    let t9064 = t1685 * t668;
    (t9052, t9055, t9056, t9058, t9060, t9062, t9064)
}
