//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1160/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1160<F: Float>(t2736: F, t32376: F, t14612: F, t4348: F, t2738: F, t1586: F) -> (F, F, F) {
    let t32377 = t32376 * t2736;
    let t32378 = t14612 * t4348;
    let t32379 = t2738 * t32378;
    let t32380 = t1586 * t32379;
    (t32377, t32379, t32380)
}
