//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1232/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1232<F: Float>(t13780: F, t13859: F, t3990: F, t9702: F, t13917: F, t3258: F, t51021: F, t51023: F, t1114: F, t50942: F, t13984: F, t3308: F, t859: F) -> (F, F, F, F, F) {
    let t53212 = t13859 * t3990 * t13780 * t9702;
    let t53227 = t13917 * t51021 * t3258 * t51023;
    let t53229 = t1114 * t50942;
    let t53230 = t53229 * t13984;
    let t53233 = t859 * t3308;
    (t53212, t53227, t53229, t53230, t53233)
}
