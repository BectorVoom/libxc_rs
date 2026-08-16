//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 608/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk608<F: Float>(t642: F, t695: F, t1060: F, t1757: F, t5192: F, t5182: F, t1801: F, t4644: F, t1800: F, t1799: F, t1755: F, t654: F) -> (F, F, F, F, F, F, F) {
    let t5193 = t642 * t695;
    let t5194 = t1060 * t1757;
    let t5195 = t5193 * t5194;
    let t5196 = t5192 * t5195;
    let t5197 = t5182 * t5196;
    let t5199 = t1801 * t4644;
    let t5200 = t1800 * t5199;
    let t5201 = t1799 * t5200;
    let t5203 = t654 * t1755;
    (t5193, t5196, t5197, t5199, t5200, t5201, t5203)
}
