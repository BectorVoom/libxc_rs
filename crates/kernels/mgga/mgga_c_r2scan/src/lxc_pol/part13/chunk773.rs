//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 773/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk773<F: Float>(t546: F, t6474: F, t560: F, t6212: F, t6211: F, t565: F, t481: F, t133: F, t2078: F, t255: F, t2168: F, t2195: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6475 = t546 * t6474;
    let t6476 = t6212 * t560;
    let t6477 = t6211 * t6476;
    let t6478 = t6475 * t6477;
    let t6480 = t565 * t6474;
    let t6481 = t6212 * t481;
    let t6482 = t6211 * t6481;
    let t6483 = t6480 * t6482;
    let t6486 = t133 * t2078 * t255;
    let t6487 = t546 * t6486;
    let t6490 = t565 * t6486;
    let t6493 = t2195 * t2168;
    (t6475, t6476, t6478, t6480, t6481, t6483, t6487, t6490, t6493)
}
