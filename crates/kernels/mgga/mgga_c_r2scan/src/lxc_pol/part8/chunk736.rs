//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 736/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk736<F: Float>(t5052: F, t95: F, t1559: F, t282: F, t283: F, t114: F, t133: F, t1541: F, t146: F, t1603: F, t2228: F) -> (F, F, F, F, F, F) {
    let t5053 = t95 * t5052;
    let t5084 = t1559 * t282;
    let t5086 = 1.0 / t283 / t5084;
    let t5087 = t114 * t5086;
    let t5094 = t1541 * t133;
    let t5095 = t146 * t5094;
    let t5100 = t2228 * t1603;
    (t5053, t5086, t5087, t5094, t5095, t5100)
}
