//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 948/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk948<F: Float>(t4859: F, t960: F, t2840: F, t4782: F, t4788: F, t1336: F, t2515: F, t4841: F, t6967: F, t4749: F, t4801: F, t4862: F) -> (F, F, F, F, F, F, F, F) {
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
