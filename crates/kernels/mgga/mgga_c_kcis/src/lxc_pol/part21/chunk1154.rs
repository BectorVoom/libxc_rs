//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1154/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1154<F: Float>(t1014: F, t27882: F, t1092: F, t1804: F, t26760: F, t3316: F, t26748: F, t27803: F, t27903: F, t44544: F, t7703: F, t27763: F, t3228: F, t27788: F, t92701: F, t1749: F, t303: F, t3191: F) -> (F, F, F, F, F, F, F, F) {
    let t96137 = t1014 * t27882;
    let t96138 = 0.33163888888888888888e-2 * t96137;
    let t96141 = t1092 * t26760 * t1804 * t3316;
    let t96148 = 0.15445601851851851852e-3 * t26748 * t27803;
    let t96150 = t7703 * t44544 * t27903;
    let t96154 = t1092 * t27763 * t1804 * t3228;
    let t96157 = t1092 * t92701 * t27788;
    let t96160 = t303 * t1749 * t3191;
    (t96137, t96138, t96141, t96148, t96150, t96154, t96157, t96160)
}
