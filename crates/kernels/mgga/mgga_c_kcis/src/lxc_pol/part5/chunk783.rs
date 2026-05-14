//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 783/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk783<F: Float>(t3217: F, t6496: F, t376: F, t375: F, t1130: F, t6555: F, t355: F, t6480: F, t381: F, t389: F, t1813: F, t5172: F, t1809: F, t1817: F, t388: F, t6486: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6700 = t3217 * t6496;
    let t6701 = t376 * t6700;
    let t6702 = t375 * t6701;
    let t6704 = t1130 * t6555;
    let t6705 = t376 * t6704;
    let t6706 = t375 * t6705;
    let t6708 = t6480 * t355;
    let t6709 = t6708 * t381;
    let t6710 = t6709 * t389;
    let t6712 = t5172 * t1813;
    let t6714 = t1809 * t1817;
    let t6716 = t388 * t6486;
    (t6700, t6701, t6702, t6704, t6705, t6706, t6708, t6709, t6710, t6712, t6714, t6716)
}
