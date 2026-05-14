//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1335/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1335<F: Float>(t481: F, t7977: F, t2148: F, t7614: F, t5100: F, t7407: F, t1234: F, t2841: F, t6243: F, t1604: F, t2207: F, t2837: F, t5181: F, t537: F, t7194: F, t113: F) -> (F, F, F, F, F, F, F) {
    let t25177 = t7977 * t481;
    let t25179 = t7614 * t2148 * t25177;
    let t25181 = t5100 * t7407;
    let t25182 = 0.29634521323209802194e0 * t25181;
    let t25183 = t2841 * t1234;
    let t25184 = t6243 * t25183;
    let t25185 = t1604 * t25184;
    let t25188 = t2207 * t2837 * t5181;
    let t25189 = 0.6112917064160653851e0 * t25188;
    let t25191 = t537 * t7194;
    let t25192 = t25191 * t113;
    (t25179, t25182, t25184, t25185, t25189, t25191, t25192)
}
