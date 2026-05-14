//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 900/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk900<F: Float>(t1988: F, t3242: F, t493: F, t1992: F, t1993: F, t3382: F, t441: F, t4680: F, t439: F, t445: F, t1972: F, t3174: F, t3173: F, t4588: F, t12035: F, t12038: F, t12040: F, t12042: F, t12047: F, t12051: F, t12055: F) -> (F, F, F, F, F, F) {
    let t12058 = t493 * t1988 * t3242 / 45.0;
    let t12062 = t493 * t1992 * t1993 * t3382 / 15.0;
    let t12063 = t441 * t4680;
    let t12066 = t439 * t12063 * t445 / 15.0;
    let t12068 = 2.0 / 9.0 * t1972 * t3174;
    let t12071 = 2.0 / 9.0 * t493 * t4588 * t3173;
    let t12072 = t12035 - t12038 - t12040 + t12042 + t12047 + t12051 - t12055 + t12058 + t12062 + t12066 - t12068 - t12071;
    (t12058, t12062, t12066, t12068, t12071, t12072)
}
