//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2245/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2245<F: Float>(t10937: F, t18041: F, t1041: F, t13969: F, t17636: F, t17642: F, t17906: F, t3117: F, t17624: F, t2960: F, t5884: F, t698: F, t973: F) -> (F, F, F, F, F, F) {
    let t62499 = t10937 * t18041;
    let t62510 = t1041 * t13969 * t17636;
    let t62515 = t1041 * t13969 * t17642;
    let t62534 = t3117 * t17906;
    let t62556 = t2960 * t17624;
    let t62559 = t973 * t698 * t5884;
    (t62499, t62510, t62515, t62534, t62556, t62559)
}
