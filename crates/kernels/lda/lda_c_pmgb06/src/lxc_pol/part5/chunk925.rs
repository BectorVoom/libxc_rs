//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 925/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk925<F: Float>(t1992: F, t3457: F, t517: F, t5312: F, t1710: F, t830: F, t500: F, t2010: F, t806: F, t1435: F, t1872: F, t132: F, t1547: F, t2107: F) -> (F, F, F, F, F, F, F) {
    let t12006 = t1992 * t3457;
    let t12012 = t5312 * t517;
    let t12036 = t830 * t1710;
    let t12037 = t12036 * t500;
    let t12038 = F::new(2.0) / F::new(135.0) * t12037;
    let t12041 = t2010 * t806;
    let t12092 = t1435 * t1872;
    let t12112 = t132 * t1547 * t2107;
    (t12006, t12012, t12036, t12038, t12041, t12092, t12112)
}
