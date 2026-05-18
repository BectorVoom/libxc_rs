//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1231/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1231<F: Float>(t19894: F, t3074: F, t6170: F, t840: F, t2353: F, t353: F, t814: F, t859: F, t2231: F, t810: F, t8599: F, t4386: F) -> (F, F, F, F, F) {
    let t21727 = t3074 * t19894;
    let t21733 = t840 * t6170;
    let t21737 = t859 * t353 * t2353 * t814;
    let t21742 = t8599 * t353 * t2231 * t810;
    let t21747 = t4386 * t353 * t2353 * t810;
    (t21727, t21733, t21737, t21742, t21747)
}
