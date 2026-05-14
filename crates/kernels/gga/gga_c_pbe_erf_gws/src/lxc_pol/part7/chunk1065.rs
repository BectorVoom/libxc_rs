//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1065/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1065<F: Float>(t21337: F, t2145: F, t6106: F, t2150: F, t2387: F, t6710: F, t6352: F, t6416: F, t2168: F, t6269: F, t6523: F, t6524: F, t3138: F, t3139: F, t6177: F, t6360: F) -> (F, F, F, F, F, F) {
    let t21338 = 7.0 / 12.0 * t21337;
    let t21339 = t6106 * t2145;
    let t21341 = t21339 * t2150 / 12.0;
    let t21346 = t2387 * t6710;
    let t21348 = t21346 * t2150 / 6.0;
    let t21350 = t6416 * t6352;
    let t21355 = 3.0 / 8.0 * t2168 * t6523 * t6269 * t6524;
    let t21359 = 3.0 / 8.0 * t3138 * t3139 * t6177 * t6360;
    (t21338, t21341, t21348, t21350, t21355, t21359)
}
