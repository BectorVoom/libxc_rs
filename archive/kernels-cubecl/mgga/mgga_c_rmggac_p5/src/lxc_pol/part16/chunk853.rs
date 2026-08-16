//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 853/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk853<F: Float>(t8452: F, t8458: F, t8494: F, t8498: F, t8505: F, t8509: F, t9300: F, t9303: F, t8513: F, t9310: F, t9312: F, t9316: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42397 = F::cast_from(0.1702583995731913576e-4_f64) * t8452;
    let t42399 = F::cast_from(0.212822999466489197e-4_f64) * t8458;
    let t42408 = F::cast_from(0.1702583995731913576e-4_f64) * t8494;
    let t42413 = F::cast_from(0.1702583995731913576e-4_f64) * t8498;
    let t42414 = F::cast_from(0.5107751987195740728e-4_f64) * t8505;
    let t42415 = F::cast_from(0.15323255961587222184e-3_f64) * t8509;
    let t42416 = F::cast_from(0.79828278012425390428e-1_f64) * t9300;
    let t42417 = F::cast_from(0.11974241701863808564e0_f64) * t9303;
    let t42418 = F::cast_from(0.1702583995731913576e-4_f64) * t8513;
    let t42420 = F::cast_from(0.4726e1_f64) * t9310;
    let t42421 = F::cast_from(0.11974241701863808564e0_f64) * t9312;
    let t42424 = F::cast_from(0.23948483403727617128e0_f64) * t9316;
    (t42397, t42399, t42408, t42413, t42414, t42415, t42416, t42417, t42418, t42420, t42421, t42424)
}
