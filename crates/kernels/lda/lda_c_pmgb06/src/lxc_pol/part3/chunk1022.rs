//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1022/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1022<F: Float>(t12154: F, t12156: F, t439: F, t1: F, t1069: F, t3092: F, t2010: F, t5260: F, t1074: F, t4667: F, t1897: F, t1420: F, t5245: F) -> (F, F, F, F, F, F) {
    let t12159 = F::new(88.0) / F::new(243.0) * t439 * t12154 * t12156;
    let t12161 = t3092 * t1 * t1069;
    let t12164 = F::new(16.0) / F::new(27.0) * t2010 * t5260 * t12161;
    let t12165 = t4667 * t1074;
    let t12168 = F::new(4.0) / F::new(15.0) * t2010 * t1897 * t12165;
    let t12170 = F::new(2.0) / F::new(3.0) * t1420 * t5245;
    (t12159, t12161, t12164, t12165, t12168, t12170)
}
