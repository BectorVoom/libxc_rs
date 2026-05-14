//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 971/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk971<F: Float>(t1915: F, t19336: F, t1981: F, t1385: F, t439: F, t477: F, t7489: F, t1897: F, t19766: F, t1901: F, t19786: F, t16612: F, t161: F, t166: F, t2093: F, t6904: F) -> (F, F, F, F, F, F) {
    let t20264 = 4.0 / 5.0 * t1981 * t1915 * t19336;
    let t20268 = 2.0 / 15.0 * t439 * t1385 * t7489 * t477;
    let t20271 = 8.0 / 15.0 * t439 * t1897 * t19766;
    let t20274 = 4.0 / 3.0 * t439 * t1901 * t19786;
    let t20275 = 2.0 / 15.0 * t16612;
    let t20279 = t161 * t166 * t2093 * t6904 / 10.0;
    (t20264, t20268, t20271, t20274, t20275, t20279)
}
