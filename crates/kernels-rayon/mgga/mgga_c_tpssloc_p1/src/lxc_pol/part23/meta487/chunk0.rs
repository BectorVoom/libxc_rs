//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1493/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1493(t54325: f64, t56168: f64, t54380: f64, t54382: f64, t20067: f64, t20077: f64, t39356: f64, t39360: f64, t39364: f64, t39373: f64, t39384: f64, t39393: f64, t39397: f64, t39400: f64, t39408: f64, t5126: f64, t6330: f64) -> (f64, f64, f64, f64, f64) {
    let t79896 = 0.22787578869697033845e-2_f64 * t54325;
    let t79897 = 0.70178683471615754484e1_f64 * t56168;
    let t79898 = 0.65061487801810439052e-1_f64 * t54380;
    let t79899 = 0.19263893255070628431e1_f64 * t54382;
    let t79903 = 36.0_f64 * t20067 * t5126 * t6330 - 36.0_f64 * t20077 * t5126 * t6330 + t39356 + t39360 + t39364 + t39373 - t39384 + t39393 - t39397 - t39400 + t39408 - t79896 + t79897 + t79898 + t79899;
    (t79896, t79897, t79898, t79899, t79903)
}
