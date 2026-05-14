//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1333/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1333<F: Float>(t114618: F, t1339: F, t6225: F, t33604: F, t6229: F, t1411: F, t25383: F, t33608: F, t114480: F, t26876: F, t3482: F, t1328: F, t8048: F, t20625: F, t2075: F, t6183: F) -> (F, F, F, F, F, F) {
    let t119351 = t1339 * t114618 * t6225;
    let t119354 = t1339 * t33604 * t6229;
    let t119357 = t1411 * t33608 * t25383;
    let t119364 = t3482 * t114480 * t26876;
    let t119366 = t1328 * t8048;
    let t119372 = t6183 * t20625 * t2075;
    (t119351, t119354, t119357, t119364, t119366, t119372)
}
