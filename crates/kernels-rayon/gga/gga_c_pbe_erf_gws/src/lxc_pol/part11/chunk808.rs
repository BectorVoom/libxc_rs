//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 808/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk808(t10841: f64, t12715: f64, t12719: f64, t12721: f64, t12725: f64, t12726: f64, t12728: f64, t12733: f64, t12735: f64, t12737: f64, t12739: f64, t12741: f64, t12744: f64, t12746: f64, t12750: f64, t5359: f64, t7573: f64) -> f64 {
    let t13021 = t12715 - t12719 + t12721 + t12725 + 0.9973633333333333333e-1_f64 * t7573 - t12726 - t12728 + t12733 - t12735 + t12737 + t12739 - t12741 + t5359 - t12744 + t12746 + 2.0_f64 / 3.0_f64 * t10841 - t12750;
    t13021
}
