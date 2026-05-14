//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 761/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk761<F: Float>(t1561: F, t1563: F, t2259: F, t498: F, t1559: F, t282: F, t283: F, t114: F, t792: F, t133: F, t1541: F, t146: F, t1543: F, t788: F, t785: F, t1603: F, t2228: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5078 = t1561 * t1563;
    let t5081 = t498 * t2259;
    let t5084 = t1559 * t282;
    let t5086 = 1.0 / t283 / t5084;
    let t5087 = t114 * t5086;
    let t5088 = t1563 * t792;
    let t5094 = t1541 * t133;
    let t5095 = t146 * t5094;
    let t5096 = t788 * t1543;
    let t5098 = t5095 * t785 * t5096;
    let t5100 = t2228 * t1603;
    (t5078, t5081, t5086, t5087, t5088, t5094, t5095, t5096, t5098, t5100)
}
