//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1014/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1014<F: Float>(t441: F, t4680: F, t439: F, t445: F, t1972: F, t3174: F, t3173: F, t4588: F, t493: F, t12035: F, t12038: F, t12040: F, t12042: F, t12047: F, t12051: F, t12055: F, t12058: F, t12062: F) -> (F, F, F, F) {
    let t12063 = t441 * t4680;
    let t12066 = t439 * t12063 * t445 / F::new(15.0);
    let t12068 = F::new(2.0) / F::new(9.0) * t1972 * t3174;
    let t12071 = F::new(2.0) / F::new(9.0) * t493 * t4588 * t3173;
    let t12072 = t12035 - t12038 - t12040 + t12042 + t12047 + t12051 - t12055 + t12058 + t12062 + t12066 - t12068 - t12071;
    (t12066, t12068, t12071, t12072)
}
