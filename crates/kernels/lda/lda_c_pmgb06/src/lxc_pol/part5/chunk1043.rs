//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1043/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1043<F: Float>(t1444: F, t7513: F, t493: F, t5447: F, t7512: F, t1438: F, t7290: F, t332: F, t1901: F, t439: F, t10431: F, t477: F, t7477: F) -> (F, F, F, F, F) {
    let t19485 = F::new(2.0) / F::new(15.0) * t1444 * t7513;
    let t19488 = F::new(2.0) / F::new(15.0) * t493 * t5447 * t7512;
    let t19489 = t1438 * t7290;
    let t19490 = t19489 * t332;
    let t19493 = t439 * t1901 * t19490 / F::new(27.0);
    let t19497 = F::new(8.0) / F::new(81.0) * t439 * t10431 * t7477 * t477;
    (t19485, t19488, t19490, t19493, t19497)
}
