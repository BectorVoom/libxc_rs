//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1370/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1370<F: Float>(t1436: F, t1439: F, t17579: F, t79: F, t13726: F, t806: F, t2007: F, t5220: F, t2012: F, t5210: F, t801: F, t1415: F, t15845: F, t496: F) -> (F, F, F, F, F) {
    let t17990 = F::new(8.0) / F::new(27.0) * t17579 * t1436 * t1439 * t79;
    let t17991 = t13726 * t806;
    let t17992 = F::new(8.0) / F::new(135.0) * t17991;
    let t17993 = t5220 * t2007;
    let t17994 = F::new(8.0) / F::new(135.0) * t17993;
    let t17996 = t801 * t5210 * t2012;
    let t17997 = F::new(4.0) / F::new(27.0) * t17996;
    let t18001 = F::new(16.0) / F::new(45.0) * t15845 * t496 * t1415 * t79;
    (t17990, t17992, t17994, t17997, t18001)
}
