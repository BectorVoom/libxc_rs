//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 892/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk892<F: Float>(t19561: F, t3802: F, t11807: F, t6331: F, t27222: F, t3123: F, t3861: F, t904: F, t3749: F, t6717: F, t20378: F, t3912: F, t11777: F, t6183: F, t20940: F, t3837: F) -> (F, F, F, F, F, F, F, F) {
    let t36814 = t3802 * t19561;
    let t36837 = t6331 * t11807;
    let t36869 = t3123 * t27222;
    let t36880 = t904 * t3861;
    let t36920 = t6717 * t3749;
    let t36962 = t3912 * t20378;
    let t37138 = t6183 * t11777;
    let t37257 = t20940 * t3837;
    (t36814, t36837, t36869, t36880, t36920, t36962, t37138, t37257)
}
