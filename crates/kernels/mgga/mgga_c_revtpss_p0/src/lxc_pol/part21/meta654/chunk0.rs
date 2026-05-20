//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2441/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2441<F: Float>(t11727: F, t3106: F, t3223: F, t3230: F, t11817: F, t3224: F, t1024: F, t11961: F, t3042: F, t3056: F, t225: F, t11274: F, t12009: F) -> (F, F, F, F, F, F, F) {
    let t42338 = t3106 * t11727;
    let t42340 = t3223 * t3230;
    let t42346 = t3224 * t11817;
    let t42355 = t1024 * t11961;
    let t42358 = t3042 * t3056;
    let t42359 = t42358 * t225;
    let t42369 = t11274 * t12009;
    (t42338, t42340, t42346, t42355, t42358, t42359, t42369)
}
