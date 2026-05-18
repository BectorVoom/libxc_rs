//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1103/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1103<F: Float>(t1915: F, t19336: F, t1981: F, t1385: F, t439: F, t477: F, t7489: F, t1897: F, t19766: F, t1901: F, t19786: F, t16612: F) -> (F, F, F, F, F) {
    let t20264 = F::new(4.0) / F::new(5.0) * t1981 * t1915 * t19336;
    let t20268 = F::new(2.0) / F::new(15.0) * t439 * t1385 * t7489 * t477;
    let t20271 = F::new(8.0) / F::new(15.0) * t439 * t1897 * t19766;
    let t20274 = F::new(4.0) / F::new(3.0) * t439 * t1901 * t19786;
    let t20275 = F::new(2.0) / F::new(15.0) * t16612;
    (t20264, t20268, t20271, t20274, t20275)
}
