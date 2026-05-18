//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1089/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1089<F: Float>(t2104: F, t5699: F, t5974: F, t2922: F, t5970: F, t2003: F, t54: F, t5695: F, t300: F, t5633: F, t178: F, t5943: F, t752: F) -> (F, F, F, F, F, F) {
    let t17797 = t2104 * t5974 * t5699;
    let t17814 = t2922 * t5974 * t5970;
    let t17848 = t54 * t2003;
    let t17850 = t2104 * t17848 * t5695;
    let t17852 = t300 * t5633;
    let t17864 = t752 * t5943 * t178;
    (t17797, t17814, t17848, t17850, t17852, t17864)
}
