//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 853/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk853(t8452: f64, t8458: f64, t8494: f64, t8498: f64, t8505: f64, t8509: f64, t9300: f64, t9303: f64, t8513: f64, t9310: f64, t9312: f64, t9316: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42397 = 0.1702583995731913576e-4_f64 * t8452;
    let t42399 = 0.212822999466489197e-4_f64 * t8458;
    let t42408 = 0.1702583995731913576e-4_f64 * t8494;
    let t42413 = 0.1702583995731913576e-4_f64 * t8498;
    let t42414 = 0.5107751987195740728e-4_f64 * t8505;
    let t42415 = 0.15323255961587222184e-3_f64 * t8509;
    let t42416 = 0.79828278012425390428e-1_f64 * t9300;
    let t42417 = 0.11974241701863808564e0_f64 * t9303;
    let t42418 = 0.1702583995731913576e-4_f64 * t8513;
    let t42420 = 0.4726e1_f64 * t9310;
    let t42421 = 0.11974241701863808564e0_f64 * t9312;
    let t42424 = 0.23948483403727617128e0_f64 * t9316;
    (t42397, t42399, t42408, t42413, t42414, t42415, t42416, t42417, t42418, t42420, t42421, t42424)
}
