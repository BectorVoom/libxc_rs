//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 785/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk785<F: Float>(t6471: F, t6472: F, t905: F, t2308: F, t2319: F, t1477: F, t855: F, t863: F, t888: F, t838: F, t864: F) -> (F, F, F, F, F) {
    let t6473 = t6471 * t6472;
    let t6474 = t905 * t6473;
    let t6477 = t2319 * t2308;
    let t6480 = t863 * t855 * t1477;
    let t6481 = t6480 * t888;
    let t6482 = F::new(35.0) / F::new(72.0) * t6481;
    let t6484 = t863 * t864 * t838;
    (t6474, t6477, t6480, t6482, t6484)
}
