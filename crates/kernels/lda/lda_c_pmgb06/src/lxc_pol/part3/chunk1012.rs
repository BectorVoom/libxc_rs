//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1012/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1012<F: Float>(t1420: F, t5350: F, t1710: F, t830: F, t500: F, t1417: F, t5194: F, t2010: F, t806: F, t497: F, t517: F, t1981: F, t496: F, t529: F) -> (F, F, F, F, F) {
    let t12035 = F::new(2.0) / F::new(5.0) * t1420 * t5350;
    let t12036 = t830 * t1710;
    let t12037 = t12036 * t500;
    let t12038 = F::new(2.0) / F::new(135.0) * t12037;
    let t12039 = t5194 * t1417;
    let t12040 = F::new(4.0) / F::new(45.0) * t12039;
    let t12041 = t2010 * t806;
    let t12042 = F::new(8.0) / F::new(1215.0) * t12041;
    let t12043 = t517 * t497;
    let t12047 = F::new(2.0) / F::new(15.0) * t1981 * t496 * t12043 * t529;
    (t12035, t12038, t12040, t12042, t12047)
}
