//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1076/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1076<F: Float>(t1731: F, t5304: F, t1730: F, t5232: F, t5257: F, t1773: F, t5255: F, t5281: F, t173: F, t1764: F, t614: F, t1736: F) -> (F, F, F, F, F, F, F, F) {
    let t17033 = t1731 * t5304;
    let t17034 = t1730 * t17033;
    let t17040 = t5257 * t5232;
    let t17043 = t1730 * t5255 * t1773;
    let t17044 = t17043 * t5281;
    let t17051 = t1764 * t173;
    let t17053 = t1730 * t17051 * t614;
    let t17054 = t17053 * t1736;
    (t17033, t17034, t17040, t17043, t17044, t17051, t17053, t17054)
}
