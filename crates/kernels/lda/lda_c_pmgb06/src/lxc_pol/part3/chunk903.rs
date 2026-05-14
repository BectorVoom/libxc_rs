//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 903/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk903<F: Float>(t12105: F, t3076: F, t802: F, t1444: F, t4757: F, t132: F, t1547: F, t2107: F, t9434: F, t9441: F, t9443: F, t9450: F, t1902: F, t3177: F, t1420: F, t5254: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12106 = 2.0 / 15.0 * t12105;
    let t12107 = t802 * t3076;
    let t12108 = t12107 / 15.0;
    let t12110 = 2.0 / 15.0 * t1444 * t4757;
    let t12112 = t132 * t1547 * t2107;
    let t12113 = t12112 / 45.0;
    let t12114 = t9434 / 15.0;
    let t12115 = t9441 / 15.0;
    let t12116 = 2.0 / 15.0 * t9443;
    let t12117 = t9450 / 15.0;
    let t12119 = t3177 * t1902 / 9.0;
    let t12121 = 2.0 / 9.0 * t1420 * t5254;
    (t12106, t12108, t12110, t12113, t12114, t12115, t12116, t12117, t12119, t12121)
}
