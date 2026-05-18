//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1118/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1118<F: Float>(t20424: F, t5068: F, t529: F, t6559: F, t9890: F, t2043: F, t2592: F, t2066: F, t1420: F, t7696: F, t439: F, t5197: F, t7695: F) -> (F, F, F, F, F, F) {
    let t20435 = F::new(2.0) / F::new(15.0) * t5068 * t6559 * t20424 * t529;
    let t20436 = F::new(4.0) / F::new(405.0) * t9890;
    let t20438 = t2592 * t2043 / F::new(10.0);
    let t20440 = t2592 * t2066 / F::new(10.0);
    let t20442 = t1420 * t7696 / F::new(5.0);
    let t20445 = t439 * t5197 * t7695 / F::new(5.0);
    (t20435, t20436, t20438, t20440, t20442, t20445)
}
