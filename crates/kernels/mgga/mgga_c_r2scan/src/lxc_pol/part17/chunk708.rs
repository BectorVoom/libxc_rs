//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 708/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk708<F: Float>(t6475: F, t6477: F, t565: F, t6474: F, t481: F, t6212: F, t6211: F, t2168: F, t2195: F, t6343: F, t551: F, t566: F, t560: F, t549: F, t110: F, t6238: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6478 = t6475 * t6477;
    let t6480 = t565 * t6474;
    let t6481 = t6212 * t481;
    let t6482 = t6211 * t6481;
    let t6483 = t6480 * t6482;
    let t6493 = t2195 * t2168;
    let t6503 = t6343 * t481;
    let t6504 = t551 * t6503;
    let t6505 = t566 * t6504;
    let t6507 = t6343 * t560;
    let t6508 = t551 * t6507;
    let t6509 = t549 * t6508;
    let t6511 = t6238 * t110;
    (t6478, t6480, t6481, t6483, t6493, t6503, t6505, t6507, t6509, t6511)
}
