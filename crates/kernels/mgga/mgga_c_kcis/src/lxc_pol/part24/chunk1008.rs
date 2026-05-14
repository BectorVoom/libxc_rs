//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1008/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1008<F: Float>(t11230: F, t1282: F, t15692: F, t1872: F, t27141: F, t28265: F, t29029: F, t29031: F, t29041: F, t29082: F, t29186: F, t29188: F, t29214: F, t437: F, t6860: F, t6879: F, t7809: F, t8108: F) -> (F,) {
    let t29216 = -6.0 * t11230 * t29188 - t1282 * t29214 + 4.0 * t15692 * t8108 - 2.0 * t1872 * t28265 + 2.0 * t27141 * t6860 + t29186 * t437 - t6879 * t7809 - t29029 + t29031 - t29041 + t29082;
    (t29216,)
}
