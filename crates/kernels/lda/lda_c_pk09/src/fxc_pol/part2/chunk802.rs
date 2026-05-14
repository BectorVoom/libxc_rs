//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 802/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk802<F: Float>(t1179: F, t2451: F, t1161: F, t4821: F, t4847: F, t4852: F, t1185: F, t5: F, t2962: F, t2964: F, t4861: F, t1180: F, t1195: F, t253: F, t4803: F, t4806: F, t4807: F, t4809: F, t4817: F, t4819: F, t4824: F, t4833: F, t9643: F, t9646: F, t9653: F, t9657: F, t9660: F) -> (F, F, F, F) {
    let t9662 = t1179 * t2451;
    let t9663 = t9662 * t1161;
    let t9664 = t4821 * t9663;
    let t9666 = t4847 * t2451;
    let t9669 = t4852 * t2451;
    let t9670 = t9669 * t1185;
    let t9673 = 5.043763671738963 * t5;
    let t9674 = 1.8058298823301853 * t2962;
    let t9675 = 15.13129101521689 * t2964;
    let t9676 = t9673 - t9674 - t9675 + t4861;
    let t9677 = t1180 * t9676;
    let t9680 = t4803 - t4806 + 1.28 * t4807 - 1.28 * t4809 + t4817 - 1.28 * t4819 + 1.28 * t4824 - t4833 + 1.28 * t9643 - 1.28 * t9646 + 1.28 * t253 * t9653 - 1.28 * t253 * t9657 - 1.28 * t9660 + 1.28 * t9664 - 1.28 * t253 * t9666 + 2.56 * t1195 * t9670 - 1.28 * t253 * t9677;
    (t9673, t9674, t9675, t9680)
}
