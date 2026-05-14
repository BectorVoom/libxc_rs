//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 917/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk917<F: Float>(t3332: F, t9296: F, t6535: F, t3610: F, t7601: F, t9292: F, t2147: F, t1055: F, t3179: F, t9380: F, t6165: F, t3308: F, t1592: F, t269: F, t3177: F, t1060: F, t783: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12533 = t3332 * t9296;
    let t12534 = t6535 * t12533;
    let t12536 = t7601 * t3610;
    let t12538 = t3332 * t9292;
    let t12539 = t2147 * t12538;
    let t12541 = t3179 * t1055;
    let t12543 = t3332 * t9380;
    let t12544 = t6165 * t12543;
    let t12547 = t3308 * t9380;
    let t12548 = t1592 * t12547;
    let t12550 = t3177 * t269;
    let t12552 = t783 * t12550 * t1060;
    (t12533, t12534, t12536, t12538, t12539, t12541, t12543, t12544, t12547, t12548, t12550, t12552)
}
