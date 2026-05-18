//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 814/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk814<F: Float>(t2911: F, t5434: F, t1013: F, t4298: F, t11899: F, t3020: F, t5186: F, t5170: F, t8688: F, t2367: F, t5097: F, t1220: F) -> (F, F, F, F, F, F, F) {
    let t15138 = t5434 * t2911;
    let t15142 = t4298 * t1013;
    let t15146 = t11899 * t1013;
    let t15167 = t5186 * t3020;
    let t15174 = t5170 * t8688;
    let t15178 = t2367 * t5097;
    let t15179 = t1220 * t15178;
    (t15138, t15142, t15146, t15167, t15174, t15178, t15179)
}
