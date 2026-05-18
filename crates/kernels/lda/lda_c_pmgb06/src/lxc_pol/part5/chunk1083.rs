//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1083/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1083<F: Float>(t2991: F, t493: F, t529: F, t7598: F, t19371: F, t5470: F, t1919: F, t19389: F, t1981: F, t1385: F, t15935: F, t439: F, t760: F) -> (F, F, F, F) {
    let t20025 = F::new(2.0) / F::new(9.0) * t493 * t2991 * t7598 * t529;
    let t20028 = F::new(32.0) / F::new(27.0) * t493 * t5470 * t19371;
    let t20031 = F::new(4.0) / F::new(3.0) * t1981 * t1919 * t19389;
    let t20035 = t439 * t1385 * t15935 * t760 / F::new(15.0);
    (t20025, t20028, t20031, t20035)
}
