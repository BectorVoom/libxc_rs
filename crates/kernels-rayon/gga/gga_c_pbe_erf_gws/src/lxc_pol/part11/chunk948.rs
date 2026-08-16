//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 948/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk948(t4859: f64, t960: f64, t2840: f64, t4782: f64, t4788: f64, t1336: f64, t2515: f64, t4841: f64, t6967: f64, t4749: f64, t4801: f64, t4862: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22636 = t4859 * t960;
    let t22641 = t2840 * t4782;
    let t22653 = t2840 * t4788;
    let t22655 = t1336 * t2515;
    let t22669 = t6967 * t4841;
    let t22674 = t2840 * t4749;
    let t22676 = t2840 * t4801;
    let t22679 = t4862 * t960;
    (t22636, t22641, t22653, t22655, t22669, t22674, t22676, t22679)
}
