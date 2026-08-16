//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 774/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk774<F: Float>(t2494: F, t3114: F, t1846: F, t2063: F, t11612: F, t2368: F, t4741: F, t2378: F, t2877: F, t4703: F, t2510: F, t3805: F) -> (F, F, F, F, F, F, F) {
    let t16208 = t3114 * t2494;
    let t16225 = t1846 * t2063;
    let t16227 = t11612 * t2063;
    let t16356 = t2368 * t4741;
    let t16389 = t2877 * t2378;
    let t16541 = t2368 * t4703;
    let t16640 = t3805 * t2510;
    (t16208, t16225, t16227, t16356, t16389, t16541, t16640)
}
