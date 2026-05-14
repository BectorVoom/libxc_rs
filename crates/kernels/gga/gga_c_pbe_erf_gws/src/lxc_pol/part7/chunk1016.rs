//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1016/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1016<F: Float>(t20466: F, t6672: F, t2195: F, t814: F, t2118: F, t2189: F, t875: F, t824: F, t343: F, t6161: F, t874: F, t6158: F, t2382: F, t6677: F, t3065: F, t6297: F, t858: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20468 = t6672 * t20466 / 4.0;
    let t20469 = t2195 * t814;
    let t20470 = t2118 * t20469;
    let t20474 = t875 * t2189;
    let t20475 = t824 * t20474;
    let t20480 = t6161 * t874 * t343;
    let t20481 = t6158 * t20480;
    let t20485 = t2382 * t6677;
    let t20487 = t3065 * t858 * t6297;
    (t20468, t20469, t20470, t20474, t20475, t20480, t20481, t20485, t20487)
}
