//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 999/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk999<F: Float>(t20411: F, t3912: F, t1076: F, t4394: F, t3772: F, t931: F, t21430: F, t3759: F, t3832: F, t6228: F, t3827: F, t6455: F) -> (F, F, F, F, F, F) {
    let t38264 = t3912 * t20411;
    let t38375 = t4394 * t1076;
    let t38451 = t3772 * t931;
    let t38506 = t21430 * t3759;
    let t38681 = t6228 * t3832;
    let t38683 = t6455 * t3827;
    (t38264, t38375, t38451, t38506, t38681, t38683)
}
