//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1112/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1112<F: Float>(t493: F, t6119: F, t6527: F, t1420: F, t7690: F, t18016: F, t439: F, t805: F, t2477: F, t5187: F, t2002: F, t6300: F) -> (F, F, F, F, F) {
    let t20367 = F::new(2.0) / F::new(5.0) * t493 * t6119 * t6527;
    let t20369 = t1420 * t7690 / F::new(15.0);
    let t20372 = t439 * t18016 * t805 / F::new(15.0);
    let t20374 = F::new(2.0) / F::new(15.0) * t5187 * t2477;
    let t20376 = F::new(2.0) / F::new(15.0) * t2002 * t6300;
    (t20367, t20369, t20372, t20374, t20376)
}
