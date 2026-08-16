//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1018/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1018(t1971: f64, t236: f64, t6105: f64, t8517: f64, t10093: f64, t3351: f64, t498: f64, t515: f64, t9210: f64, t321: f64, t7248: f64, t2144: f64, t333: f64, t7231: f64) -> (f64, f64, f64, f64) {
    let t47275 = t8517 * t1971 * t236 * t6105;
    let t47280 = t3351 * t9210 * t515 * t10093 * t498;
    let t47287 = t3351 * t7248 * t515 * t10093 * t321;
    let t47292 = t3351 * t7231 * t2144 * t10093 * t333;
    (t47275, t47280, t47287, t47292)
}
