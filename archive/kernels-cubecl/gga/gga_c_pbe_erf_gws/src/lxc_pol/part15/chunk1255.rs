//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1255/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1255<F: Float>(t1112: F, t13918: F, t361: F, t13917: F, t6639: F, t14424: F, t9381: F, t353: F, t4183: F, t814: F, t859: F, t52915: F, t9521: F) -> (F, F, F, F, F) {
    let t53446 = t13918 * t1112;
    let t53447 = t361 * t53446;
    let t53449 = t13917 * t53447 * t6639;
    let t53460 = t13917 * t14424 * t9381;
    let t53464 = t859 * t353 * t4183 * t814;
    let t53468 = t13917 * t52915 * t9521;
    (t53447, t53449, t53460, t53464, t53468)
}
