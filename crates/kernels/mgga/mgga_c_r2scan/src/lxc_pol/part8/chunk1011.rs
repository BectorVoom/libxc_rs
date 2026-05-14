//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1011/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1011<F: Float>(t7032: F, t4827: F, t4839: F, t4842: F, t4845: F, t4996: F, t5000: F, t5004: F, t5008: F, t5020: F, t5022: F, t9911: F, t7051: F, t7054: F, t7107: F, t7109: F) -> (F, F, F, F, F, F) {
    let t9912 = 0.73245789224026180216e-3 * t7032;
    let t9913 = t4996 - t5000 - t5004 - t5008 - t4827 + t4839 - t5020 + t4842 - t5022 + t9911 + t9912 - t4845;
    let t9914 = 24.0 * t7051;
    let t9915 = 24.0 * t7054;
    let t9916 = 96.0 * t7107;
    let t9917 = 60.0 * t7109;
    (t9912, t9913, t9914, t9915, t9916, t9917)
}
