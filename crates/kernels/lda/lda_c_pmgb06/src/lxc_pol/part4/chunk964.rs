//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 964/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk964<F: Float>(t2010: F, t806: F, t497: F, t517: F, t441: F, t4680: F, t1447: F, t4762: F, t1423: F, t5198: F, t1435: F, t1872: F, t1517: F, t1887: F, t3076: F, t802: F) -> (F, F, F, F, F, F, F, F) {
    let t12041 = t2010 * t806;
    let t12043 = t517 * t497;
    let t12063 = t441 * t4680;
    let t12075 = t1447 * t4762;
    let t12084 = t1423 * t5198;
    let t12092 = t1435 * t1872;
    let t12105 = t1887 * t1517;
    let t12107 = t802 * t3076;
    (t12041, t12043, t12063, t12075, t12084, t12092, t12105, t12107)
}
