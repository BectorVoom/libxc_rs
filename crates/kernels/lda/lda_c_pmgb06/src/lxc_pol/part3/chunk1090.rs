//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1090/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1090<F: Float>(t9828: F, t9830: F, t9832: F, t9834: F, t9837: F, t9847: F, t9853: F, t12514: F, t1461: F, t5065: F, t5140: F, t2987: F, t5068: F, t5090: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12973 = F::new(4.0) / F::new(135.0) * t9828;
    let t12974 = F::new(2.0) / F::new(45.0) * t9830;
    let t12975 = F::new(2.0) / F::new(45.0) * t9832;
    let t12976 = F::new(4.0) / F::new(45.0) * t9834;
    let t12977 = F::new(2.0) / F::new(45.0) * t9837;
    let t12978 = F::new(4.0) / F::new(45.0) * t9847;
    let t12979 = F::new(4.0) / F::new(45.0) * t9853;
    let t12981 = t5065 * t12514 * t1461;
    let t12982 = t12981 * t5140;
    let t12983 = F::new(4.0) / F::new(27.0) * t12982;
    let t12986 = F::new(2.0) / F::new(15.0) * t5068 * t5090 * t2987;
    (t12973, t12974, t12975, t12976, t12977, t12978, t12979, t12983, t12986)
}
