//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 943/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk943<F: Float>(t6887: F, t766: F, t2330: F, t2333: F, t2266: F, t481: F, t795: F, t2332: F, t287: F, t106: F, t97: F, t1048: F, t2262: F, t3352: F, t372: F, t4845: F, t4873: F, t5024: F, t5026: F, t5028: F, t5030: F, t5033: F, t5035: F, t5039: F, t5879: F, t6801: F, t6881: F, t6884: F, t6885: F) -> (F, F, F, F, F, F, F, F) {
    let t6888 = t6887 * t766;
    let t6890 = t2330 * t2333;
    let t6892 = t2266 * t6890 * t481;
    let t6893 = 9.0 * t6892;
    let t6894 = t2330 * t795;
    let t6897 = 1.0 / t2332 / t287;
    let t6899 = t97 * t106 * t6894 * t6897;
    let t6900 = 2.0 * t6899;
    let t6902 = t1048 * t3352 * t2262;
    let t6903 = 3.0 * t6902;
    let t6904 = t4845 - t5024 + 3.0 * t6801 + t372 * t5879 + t6881 - t6884 - 0.14178e2 * t6885 - 0.7089e1 * t6888 + t6893 - t5026 + t5028 + t5030 - t4873 - t6900 - t5033 + t6903 - t5035 - t5039;
    (t6888, t6890, t6893, t6894, t6897, t6900, t6903, t6904)
}
