//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 714/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk714<F: Float>(t2911: F, t8470: F, t2899: F, t426: F, t425: F, t462: F, t2886: F, t458: F, t8465: F, t2879: F, t119: F, t492: F) -> (F, F, F, F, F) {
    let t8471 = t2911 * t8470;
    let t8473 = t426 * t2899;
    let t8475 = t462 * t425;
    let t8476 = t8475 * t2886;
    let t8478 = t8465 * t458;
    let t8479 = t2879 * t8478;
    let t8482 = t492 * t119;
    (t8471, t8473, t8476, t8479, t8482)
}
