//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1328/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1328<F: Float>(t114618: F, t1411: F, t33596: F, t33600: F, t33604: F, t1440: F, t33608: F, t8247: F, t34795: F, t9442: F, t20233: F, t34762: F, t32087: F, t114480: F, t26910: F, t5633: F) -> (F, F, F, F, F, F, F) {
    let t119254 = t1411 * t114618 * t33596;
    let t119257 = t1411 * t33604 * t33600;
    let t119261 = t1411 * t33608 * t8247 * t1440;
    let t119264 = t34795 * t9442;
    let t119268 = t20233 * t34762;
    let t119269 = t32087 * t119268;
    let t119272 = t5633 * t114480 * t26910;
    (t119254, t119257, t119261, t119264, t119268, t119269, t119272)
}
