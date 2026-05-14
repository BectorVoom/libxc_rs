//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 703/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk703<F: Float>(t2378: F, t2877: F, t2368: F, t4703: F, t2510: F, t3805: F, t10473: F, t2474: F, t1797: F, t2507: F, t1336: F, t140: F, t2522: F, t3517: F, t2518: F, t1814: F, t2372: F) -> (F, F, F, F, F, F, F, F) {
    let t16389 = t2877 * t2378;
    let t16541 = t2368 * t4703;
    let t16640 = t3805 * t2510;
    let t16658 = t10473 * t2474;
    let t16674 = t1797 * t2507;
    let t16676 = t140 * t1336 * t16674;
    let t16879 = t3517 * t2522;
    let t16885 = t3517 * t2518;
    let t16892 = t1814 * t2372;
    (t16389, t16541, t16640, t16658, t16676, t16879, t16885, t16892)
}
