//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1077/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1077<F: Float>(t193: F, t3938: F, t6061: F, t6109: F, t743: F, t1154: F, t24395: F, t13698: F, t1424: F, t24448: F, t2476: F, t6837: F, t1882: F, t27860: F, t13863: F, t96970: F) -> (F, F, F, F, F, F, F) {
    let t108376 = t6109 * t193 * t743 * t6061 * t3938;
    let t108381 = t6109 * t193 * t743 * t24395 * t1154;
    let t108386 = t6109 * t193 * t743 * t1424 * t13698;
    let t108391 = t24448 * t193 * t743 * t6837 * t2476;
    let t108393 = t1882 * t27860;
    let t108394 = 4.0 / 9.0 * t108393;
    let t108395 = t96970 * t13863;
    (t108376, t108381, t108386, t108391, t108393, t108394, t108395)
}
