//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1200/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1200<F: Float>(t1160: F, t17927: F, t284: F, t55330: F, t9102: F, t9104: F, t1179: F, t54615: F, t1162: F, t17666: F, t2367: F, t9116: F, t9118: F) -> (F, F, F, F, F) {
    let t55337 = t1160 * t17927 * t284;
    let t55341 = t9102 * t55330 * t9104;
    let t55343 = t1179 * t54615;
    let t55346 = t1162 * t2367 * t17666;
    let t55361 = t9116 * t55330 * t9118;
    (t55337, t55341, t55343, t55346, t55361)
}
