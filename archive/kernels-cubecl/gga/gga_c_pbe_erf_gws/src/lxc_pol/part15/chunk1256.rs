//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1256/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1256<F: Float>(t20154: F, t2376: F, t4155: F, t814: F, t50998: F, t53447: F, t6278: F, t13917: F, t14423: F, t2157: F, t2249: F, t9640: F) -> (F, F, F) {
    let t53472 = t20154 * t2376 * t4155 * t814;
    let t53476 = t50998 * t53447 * t6278;
    let t53481 = t13917 * t2249 * t14423 * t2157 * t9640;
    (t53472, t53476, t53481)
}
