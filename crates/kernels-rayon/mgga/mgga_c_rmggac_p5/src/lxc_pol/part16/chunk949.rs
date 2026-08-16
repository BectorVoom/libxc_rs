//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 949/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk949(t10088: f64, t2144: f64, t3351: f64, t352: f64, t7231: f64, t1704: f64, t2084: f64, t27: f64, t7282: f64, t30400: f64, t3352: f64, t515: f64) -> (f64, f64, f64) {
    let t45788 = t3351 * t7231 * t2144 * t10088 * t352;
    let t45794 = t7282 * t27 * t2084 * t1704;
    let t45811 = t3351 * t3352 * t515 * t30400;
    (t45788, t45794, t45811)
}
