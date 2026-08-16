//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1241/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1241<F: Float>(t1112: F, t13918: F, t361: F, t13917: F, t6639: F, t14424: F, t9381: F, t52915: F, t9521: F, t50998: F, t6278: F, t14423: F, t2157: F, t2249: F, t9640: F) -> (F, F, F, F, F) {
    let t53446 = t13918 * t1112;
    let t53447 = t361 * t53446;
    let t53449 = t13917 * t53447 * t6639;
    let t53460 = t13917 * t14424 * t9381;
    let t53468 = t13917 * t52915 * t9521;
    let t53476 = t50998 * t53447 * t6278;
    let t53481 = t13917 * t2249 * t14423 * t2157 * t9640;
    (t53449, t53460, t53468, t53476, t53481)
}
