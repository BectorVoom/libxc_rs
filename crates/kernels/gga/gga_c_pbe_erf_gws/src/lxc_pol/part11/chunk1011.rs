//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1011/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1011<F: Float>(t2756: F, t3399: F, t10878: F, t2741: F, t12710: F, t582: F, t616: F, t11019: F, t7527: F, t2749: F, t3493: F, t12517: F, t184: F, t202: F) -> (F, F, F, F, F, F) {
    let t40766 = t3399 * t2756;
    let t40768 = t2741 * t10878;
    let t40771 = t616 * t582 * t12710;
    let t40773 = t7527 * t11019;
    let t40783 = t3493 * t2749;
    let t40790 = t202 * t12517 * t184;
    (t40766, t40768, t40771, t40773, t40783, t40790)
}
