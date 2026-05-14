//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 989/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk989<F: Float>(t25450: F, t5907: F, t1175: F, t3539: F, t7744: F, t1364: F, t3544: F, t25432: F, t25437: F, t5895: F, t19399: F, t2191: F, t3564: F, t5703: F, t5932: F, t3521: F, t7854: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26667 = t5907 * t25450;
    let t26671 = t3539 * t7744 * t1175;
    let t26675 = t3544 * t7744 * t1364;
    let t26678 = t5907 * t25432;
    let t26681 = t5895 * t25437;
    let t26684 = t19399 * t2191;
    let t26685 = t3564 * t26684;
    let t26688 = t5932 * t5703;
    let t26689 = t3564 * t26688;
    let t26692 = t3521 * t7854;
    (t26667, t26671, t26675, t26678, t26681, t26684, t26685, t26688, t26689, t26692)
}
