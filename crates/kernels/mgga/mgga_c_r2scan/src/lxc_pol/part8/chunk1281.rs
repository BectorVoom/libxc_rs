//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1281/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1281<F: Float>(t2155: F, t29765: F, t1568: F, t29946: F, t7623: F, t2147: F, t6398: F, t9445: F, t113: F, t27661: F, t2115: F, t560: F, t8773: F, t2148: F, t24063: F, t481: F) -> (F, F, F, F, F, F, F) {
    let t30033 = t2155 * t29765;
    let t30038 = t7623 * t1568 * t29946;
    let t30047 = t2147 * t6398 * t9445;
    let t30049 = t27661 * t113;
    let t30050 = t2115 * t30049;
    let t30051 = t2155 * t30050;
    let t30053 = t8773 * t560;
    let t30055 = t24063 * t2148 * t30053;
    let t30057 = t8773 * t481;
    (t30033, t30038, t30047, t30050, t30051, t30055, t30057)
}
