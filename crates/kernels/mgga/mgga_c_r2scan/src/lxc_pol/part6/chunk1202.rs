//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1202/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1202<F: Float>(t234: F, t5266: F, t5271: F, t712: F, t1859: F, t1862: F, t4958: F, t5255: F, t5322: F, t5326: F, t159: F, t5246: F, t607: F, t1375: F, t5249: F, t5252: F) -> (F, F, F, F, F, F) {
    let t22152 = 0.36433041676861022416e6 * t234 * t5266 * t712 * t5271;
    let t22154 = t1859 * t4958 * t1862;
    let t22156 = t5255 * t5322;
    let t22158 = t5255 * t5326;
    let t22161 = t159 * t607 * t5246;
    let t22164 = t5249 * t1375 * t5252;
    (t22152, t22154, t22156, t22158, t22161, t22164)
}
