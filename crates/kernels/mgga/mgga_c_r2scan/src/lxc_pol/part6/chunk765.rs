//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 765/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk765<F: Float>(t5109: F, t5110: F, t1553: F, t537: F, t113: F, t2115: F, t1604: F, t489: F, t57: F, t2224: F, t514: F, t1620: F, t2215: F, t2214: F, t2232: F, t2252: F, t788: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5111 = t5109 * t5110;
    let t5114 = t537 * t1553;
    let t5115 = t5114 * t113;
    let t5116 = t2115 * t5115;
    let t5117 = t1604 * t5116;
    let t5119 = t57 * t489;
    let t5120 = t5119 * t2224;
    let t5121 = t514 * t5120;
    let t5123 = t1620 * t2215;
    let t5125 = t2214 * t2232;
    let t5126 = t514 * t5125;
    let t5128 = t788 * t2252;
    (t5111, t5115, t5116, t5117, t5119, t5120, t5121, t5123, t5125, t5126, t5128)
}
