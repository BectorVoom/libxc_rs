//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1205/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1205<F: Float>(t1356: F, t16330: F, t2072: F, t4355: F, t1571: F, t6098: F, t2080: F, t4350: F, t4332: F, t6101: F, t4358: F, t6097: F, t12732: F, t2079: F, t12689: F, t12730: F, t12761: F, t12767: F, t12772: F, t1577: F, t4331: F, t4351: F, t4356: F, t4359: F, t4363: F, t6075: F, t6080: F, t6102: F, t6114: F) -> (F,) {
    let t17792 = t16330 * t1356;
    let t17797 = t2072 * t4355;
    let t17806 = t6098 * t1571;
    let t17809 = t2080 * t4350;
    let t17812 = t6101 * t4332;
    let t17815 = t6097 * t4358;
    let t17816 = t17815 * t1571;
    let t17819 = t6101 * t4350;
    let t17822 = t2079 * t12732;
    let t17823 = t17822 * t4332;
    let t17826 = 0.11696446794910408142e1 * t4363 * t6114 + 0.58482233974552040708e0 * t1577 * t17792 + 1.0 * t6075 * t4351 + 0.32164683177870697974e2 * t17797 * t4359 + 1.0 * t12767 * t2080 - 4.0 * t12761 * t6080 + 0.64329366355741395948e2 * t12772 * t6102 - 4.0 * t4331 * t17806 - 2.0 * t4331 * t17809 - 0.19298809906722418785e3 * t12689 * t17812 + 0.64329366355741395948e2 * t4356 * t17816 + 0.32164683177870697974e2 * t4356 * t17819 + 0.20691336878655965246e4 * t12730 * t17823;
    (t17826,)
}
