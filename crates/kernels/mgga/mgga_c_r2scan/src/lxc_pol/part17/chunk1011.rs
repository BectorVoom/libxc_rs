//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1011/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1011<F: Float>(t12523: F, t3295: F, t2124: F, t9376: F, t3332: F, t9445: F, t2147: F, t9296: F, t6535: F, t3610: F, t7601: F, t9292: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12524 = t3295 * t12523;
    let t12526 = t2124 * t9376;
    let t12527 = t3295 * t12526;
    let t12529 = t3332 * t9445;
    let t12530 = t2147 * t12529;
    let t12533 = t3332 * t9296;
    let t12534 = t6535 * t12533;
    let t12536 = t7601 * t3610;
    let t12538 = t3332 * t9292;
    (t12524, t12526, t12527, t12529, t12530, t12533, t12534, t12536, t12538)
}
