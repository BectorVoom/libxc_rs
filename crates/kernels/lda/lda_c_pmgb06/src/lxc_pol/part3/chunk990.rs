//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 990/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk990<F: Float>(t1972: F, t3251: F, t835: F, t9370: F, t1977: F, t3198: F, t1498: F, t1980: F, t1983: F, t13444: F, t13447: F, t13450: F, t13453: F, t13455: F, t13456: F, t13457: F, t13461: F) -> (F, F, F, F, F) {
    let t13463 = 8.0 / 81.0 * t1972 * t3251;
    let t13465 = t9370 * t835 / 45.0;
    let t13467 = t3198 * t1977 / 15.0;
    let t13470 = 2.0 / 15.0 * t1498 * t1980 * t1983;
    let t13471 = t13444 + t13447 / 3.0 + 0.18233333333333332 * t13450 + t13453 - t13455 + t13456 + t13457 + t13461 + t13463 + t13465 + t13467 - t13470;
    (t13463, t13465, t13467, t13470, t13471)
}
