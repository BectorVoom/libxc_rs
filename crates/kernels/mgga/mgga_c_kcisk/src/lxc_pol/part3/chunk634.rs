//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 634/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk634<F: Float>(t156: F, t3122: F, t3530: F, t459: F, t1422: F, t119: F, t179: F, t1173: F, t416: F, t458: F, t1273: F, t4129: F) -> (F, F, F, F, F, F, F) {
    let t5827 = t156 * t3122;
    let t5895 = t3530 * t459;
    let t5907 = t1422 * t459;
    let t5911 = t179 * t119;
    let t5926 = t416 * t1173;
    let t5953 = t416 * t458;
    let t6125 = t4129 * t1273;
    (t5827, t5895, t5907, t5911, t5926, t5953, t6125)
}
