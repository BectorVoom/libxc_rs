//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1096/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1096<F: Float>(t2352: F, t6781: F, t829: F, t830: F, t4394: F, t745: F, t825: F, t2219: F, t898: F, t938: F, t2365: F, t4395: F) -> (F, F, F, F, F, F) {
    let t19621 = t6781 * t2352;
    let t19623 = t829 * t830 * t19621;
    let t19626 = t4394 * t745;
    let t19627 = t19626 * t825;
    let t19631 = t2219 * t898;
    let t19632 = t19631 * t938;
    let t19634 = t829 * t830 * t19632;
    let t19637 = t4395 * t2365;
    (t19623, t19626, t19627, t19631, t19634, t19637)
}
