//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 922/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk922(t10084: f64, t16156: f64, t3351: f64, t3352: f64, t44713: f64, t515: f64, t7720: f64, t9795: f64, t10072: f64, t7244: f64, t5542: f64, t9734: f64) -> (f64, f64, f64, f64, f64) {
    let t45333 = t16156 * t10084;
    let t45337 = t3351 * t3352 * t515 * t44713;
    let t45339 = t7720 * t9795;
    let t45341 = t7244 * t10072;
    let t45343 = t9734 * t5542;
    (t45333, t45337, t45339, t45341, t45343)
}
