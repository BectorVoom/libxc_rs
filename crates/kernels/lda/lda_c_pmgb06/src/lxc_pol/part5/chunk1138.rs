//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1138/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1138<F: Float>(t1447: F, t7660: F, t2465: F, t493: F, t5312: F, t17372: F, t17374: F, t1972: F, t6120: F, t17376: F, t2002: F, t6556: F) -> (F, F, F, F, F, F, F) {
    let t20662 = t1447 * t7660;
    let t20663 = F::new(4.0) / F::new(27.0) * t20662;
    let t20666 = t493 * t5312 * t2465 / F::new(15.0);
    let t20667 = F::new(2.0) / F::new(45.0) * t17372;
    let t20668 = t17374 / F::new(45.0);
    let t20670 = F::new(2.0) / F::new(5.0) * t1972 * t6120;
    let t20671 = F::new(2.0) / F::new(27.0) * t17376;
    let t20673 = t2002 * t6556 / F::new(5.0);
    (t20663, t20666, t20667, t20668, t20670, t20671, t20673)
}
