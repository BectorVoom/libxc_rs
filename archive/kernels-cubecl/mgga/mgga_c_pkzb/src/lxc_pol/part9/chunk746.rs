//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 746/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk746<F: Float>(t1708: F, t5221: F, t614: F, t95: F, t149: F, t50: F, t5181: F, t581: F, t164: F, t1753: F, t179: F, t568: F) -> (F, F, F, F, F) {
    let t5222 = t5221 * t1708;
    let t5224 = t95 * t614;
    let t5225 = t149 * t5224;
    let t5227 = t581 * t50 * t5181;
    let t5230 = t1753 * t164;
    let t5232 = t179 * t5230 * t568;
    (t5222, t5224, t5225, t5227, t5232)
}
