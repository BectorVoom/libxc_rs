//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 463/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk463<F: Float>(t2840: F, t472: F, t414: F, t960: F, t409: F, t140: F, t1503: F, t142: F, t967: F, t485: F, t971: F, t395: F) -> (F, F, F, F, F, F, F) {
    let t2841 = t2840 * t472;
    let t2843 = t414 * t960;
    let t2845 = t409 * t960;
    let t2857 = t1503 * t140;
    let t2858 = t142 * t967;
    let t2863 = t485 * t971;
    let t2864 = t2863 * t395;
    (t2841, t2843, t2845, t2857, t2858, t2863, t2864)
}
