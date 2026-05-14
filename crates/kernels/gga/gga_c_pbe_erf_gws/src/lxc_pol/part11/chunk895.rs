//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 895/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk895<F: Float>(t11413: F, t4413: F, t3802: F, t6469: F, t11629: F, t6183: F, t11786: F, t3783: F, t6616: F, t20411: F, t3912: F, t1076: F, t4394: F, t3772: F, t931: F, t21430: F, t3759: F) -> (F, F, F, F, F, F, F, F, F) {
    let t37997 = t4413 * t11413;
    let t38036 = t6469 * t3802;
    let t38063 = t6183 * t11629;
    let t38143 = t6183 * t11786;
    let t38234 = t3783 * t6616;
    let t38264 = t3912 * t20411;
    let t38375 = t4394 * t1076;
    let t38451 = t3772 * t931;
    let t38506 = t21430 * t3759;
    (t37997, t38036, t38063, t38143, t38234, t38264, t38375, t38451, t38506)
}
