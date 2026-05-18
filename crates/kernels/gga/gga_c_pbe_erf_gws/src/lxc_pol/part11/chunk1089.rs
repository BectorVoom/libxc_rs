//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1089/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1089<F: Float>(t31785: F, t31801: F, t31803: F, t31805: F, t40245: F, t40247: F, t40251: F, t40253: F, t40255: F, t40260: F, t40262: F, t40264: F) -> F {
    let t47500 = -F::new(0.95977777777777777776e-1) * t31785 - F::new(0.14814814814814814815e-1) * t31801 - F::new(0.44444444444444444445e-1) * t31803 - F::new(0.63985185185185185184e-1) * t31805 - F::new(0.10666666666666666667e0) * t40245 + F::new(0.17777777777777777778e-1) * t40247 + F::new(0.53320987654320987654e-1) * t40251 + F::new(0.10666666666666666667e0) * t40253 + F::new(0.47988888888888888888e-1) * t40255 - F::new(0.35555555555555555556e-1) * t40260 - F::new(0.19195555555555555556e0) * t40262 + F::new(0.28793333333333333333e0) * t40264;
    t47500
}
