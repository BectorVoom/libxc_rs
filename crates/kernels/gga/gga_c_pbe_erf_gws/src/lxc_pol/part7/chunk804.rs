//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 804/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk804<F: Float>(t2302: F, t2323: F, t56: F, t931: F, t19: F, t6385: F, t858: F, t884: F, t4394: F, t820: F, t274: F, t6161: F) -> (F, F, F, F, F, F, F) {
    let t6656 = t2323 * t2302;
    let t6658 = t56 * t931;
    let t6659 = t6658 * t19;
    let t6661 = t6659 * t858 * t6385;
    let t6663 = t884 * t6661 / F::new(4.0);
    let t6664 = t820 * t4394;
    let t6665 = t274 * t6161;
    (t6656, t6658, t6659, t6661, t6663, t6664, t6665)
}
