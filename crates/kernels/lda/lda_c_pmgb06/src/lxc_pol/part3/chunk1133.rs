//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1133/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1133<F: Float>(t1498: F, t1980: F, t1983: F, t13444: F, t13447: F, t13450: F, t13453: F, t13455: F, t13456: F, t13457: F, t13461: F, t13463: F, t13465: F, t13467: F) -> (F, F) {
    let t13470 = F::new(2.0) / F::new(15.0) * t1498 * t1980 * t1983;
    let t13471 = t13444 + t13447 / F::new(3.0) + F::new(0.18233333333333332) * t13450 + t13453 - t13455 + t13456 + t13457 + t13461 + t13463 + t13465 + t13467 - t13470;
    (t13470, t13471)
}
