//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 771/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk771<F: Float>(t1013: F, t11899: F, t3020: F, t5186: F, t5170: F, t8688: F, t2367: F, t5097: F, t1220: F, t4536: F, t4539: F, t1214: F, t5474: F, t1213: F, t5440: F, t490: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15146 = t11899 * t1013;
    let t15167 = t5186 * t3020;
    let t15174 = t5170 * t8688;
    let t15178 = t2367 * t5097;
    let t15179 = t1220 * t15178;
    let t15181 = t4536 * t4539;
    let t15200 = t5474 * t1214;
    let t15204 = t5440 * t1213;
    let t15205 = t490 * t15204;
    (t15146, t15167, t15174, t15178, t15179, t15181, t15200, t15204, t15205)
}
