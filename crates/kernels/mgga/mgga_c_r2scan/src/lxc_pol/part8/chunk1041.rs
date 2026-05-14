//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1041/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1041<F: Float>(t4827: F, t4839: F, t4842: F, t4845: F, t4996: F, t5000: F, t5004: F, t5008: F, t5020: F, t5022: F, t6010: F, t7878: F, t9911: F, t9912: F, t9914: F, t9915: F) -> (F,) {
    let t10295 = 0.4051561992e0 * t7878 - t4996 + t5000 + t5004 + t5008 + t4827 - t4839 + t5020 + t6010 - t4842 + t5022 - t9911 - t9912 + t4845 + t9914 + t9915;
    (t10295,)
}
