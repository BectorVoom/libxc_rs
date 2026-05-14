//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 840/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk840<F: Float>(t166: F, t6006: F, t6007: F, t2055: F, t2056: F, t607: F, t4959: F, t1376: F, t4827: F, t4839: F, t4842: F, t4996: F, t5000: F, t5004: F, t5008: F, t5010: F, t5013: F, t5016: F, t5020: F, t5022: F, t765: F) -> (F, F, F, F, F) {
    let t6010 = 0.1714584e0 * t6006 * t166 * t6007;
    let t6012 = t2055 * t607 * t2056;
    let t6014 = t4959 * t166;
    let t6017 = t1376 * t607;
    let t6020 = -t4996 + t5000 + t5004 + t5008 + t5010 + t4827 - t4839 + t5013 - t5016 + t5020 + t6010 - 0.1714584e0 * t6012 - t4842 + 0.675260332e-1 * t765 * t6014 + 0.2025780996e0 * t765 * t6017 - t5022;
    (t6010, t6012, t6014, t6017, t6020)
}
