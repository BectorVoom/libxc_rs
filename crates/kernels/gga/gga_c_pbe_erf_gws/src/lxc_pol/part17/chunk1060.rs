//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1060/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1060<F: Float>(t2195: F, t2345: F, t3240: F, t2264: F, t899: F, t923: F, t3249: F, t3219: F, t3235: F, t6636: F, t6684: F, t8884: F, t904: F) -> (F, F, F, F, F) {
    let t9626 = t2345 * t3240 * t2195;
    let t9630 = t899 * t2264 * t923;
    let t9632 = F::new(7.0) / F::new(384.0) * t9630 * t3249;
    let t9634 = t3235 * t3219 * t2195;
    let t9637 = t6684 * t6636;
    let t9638 = t904 * t8884;
    (t9626, t9632, t9634, t9637, t9638)
}
