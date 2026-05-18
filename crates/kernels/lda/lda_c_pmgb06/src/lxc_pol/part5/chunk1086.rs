//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1086/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1086<F: Float>(t1420: F, t7667: F, t1426: F, t439: F, t7666: F, t1444: F, t7671: F, t1962: F, t6244: F, t7577: F, t12092: F, t2484: F) -> (F, F, F, F, F, F) {
    let t20071 = F::new(2.0) / F::new(15.0) * t1420 * t7667;
    let t20074 = F::new(2.0) / F::new(15.0) * t439 * t1426 * t7666;
    let t20076 = F::new(2.0) / F::new(15.0) * t1444 * t7671;
    let t20079 = t439 * t1962 * t6244 / F::new(15.0);
    let t20081 = t1420 * t7577 / F::new(9.0);
    let t20084 = t439 * t12092 * t2484 / F::new(9.0);
    (t20071, t20074, t20076, t20079, t20081, t20084)
}
