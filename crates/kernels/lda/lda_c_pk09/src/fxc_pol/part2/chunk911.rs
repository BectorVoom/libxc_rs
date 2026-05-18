//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 911/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk911<F: Float>(t2964: F, t4861: F, t9673: F, t9674: F, t1180: F, t1195: F, t253: F, t4803: F, t4806: F, t4807: F, t4809: F, t4817: F, t4819: F, t4824: F, t4833: F, t9643: F, t9646: F, t9653: F, t9657: F, t9660: F, t9664: F, t9666: F, t9670: F) -> (F, F) {
    let t9675 = F::new(15.13129101521689) * t2964;
    let t9676 = t9673 - t9674 - t9675 + t4861;
    let t9677 = t1180 * t9676;
    let t9680 = t4803 - t4806 + F::new(1.28) * t4807 - F::new(1.28) * t4809 + t4817 - F::new(1.28) * t4819 + F::new(1.28) * t4824 - t4833 + F::new(1.28) * t9643 - F::new(1.28) * t9646 + F::new(1.28) * t253 * t9653 - F::new(1.28) * t253 * t9657 - F::new(1.28) * t9660 + F::new(1.28) * t9664 - F::new(1.28) * t253 * t9666 + F::new(2.56) * t1195 * t9670 - F::new(1.28) * t253 * t9677;
    (t9675, t9680)
}
