//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 972/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk972<F: Float>(t3431: F, t5272: F, t3409: F, t5213: F, t1982: F, t4254: F, t1036: F, t1095: F, t1524: F, t398: F, t864: F, t1434: F, t3770: F) -> (F, F, F, F, F) {
    let t15796 = t3431 * t5272;
    let t15807 = t3409 * t5213;
    let t15814 = t4254 * t1982;
    let t15826 = t1036 * t398 * t1095 * t1524 * t864;
    let t15828 = t3770 * t1434;
    (t15796, t15807, t15814, t15826, t15828)
}
