//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 994/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk994<F: Float>(t3788: F, t6616: F, t3810: F, t6717: F, t19561: F, t3802: F, t11807: F, t6331: F, t27222: F, t3123: F, t3861: F, t904: F) -> (F, F, F, F, F, F) {
    let t36699 = t3788 * t6616;
    let t36803 = t6717 * t3810;
    let t36814 = t3802 * t19561;
    let t36837 = t6331 * t11807;
    let t36869 = t3123 * t27222;
    let t36880 = t904 * t3861;
    (t36699, t36803, t36814, t36837, t36869, t36880)
}
