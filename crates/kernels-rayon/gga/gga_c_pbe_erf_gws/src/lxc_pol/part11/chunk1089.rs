//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1089/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1089(t31785: f64, t31801: f64, t31803: f64, t31805: f64, t40245: f64, t40247: f64, t40251: f64, t40253: f64, t40255: f64, t40260: f64, t40262: f64, t40264: f64) -> f64 {
    let t47500 = -0.95977777777777777776e-1_f64 * t31785 - 0.14814814814814814815e-1_f64 * t31801 - 0.44444444444444444445e-1_f64 * t31803 - 0.63985185185185185184e-1_f64 * t31805 - 0.10666666666666666667e0_f64 * t40245 + 0.17777777777777777778e-1_f64 * t40247 + 0.53320987654320987654e-1_f64 * t40251 + 0.10666666666666666667e0_f64 * t40253 + 0.47988888888888888888e-1_f64 * t40255 - 0.35555555555555555556e-1_f64 * t40260 - 0.19195555555555555556e0_f64 * t40262 + 0.28793333333333333333e0_f64 * t40264;
    t47500
}
