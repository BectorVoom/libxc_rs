//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 524/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk524<F: Float>(t1374: F, t1379: F, t1412: F, t188: F, t2346: F, t2499: F, t2503: F, t2522: F, t2523: F, t2524: F, t2525: F, t2676: F) -> F {
    let t2680 = t1374 + t1379 - t2499 - t2503 + F::new(4.0) / F::new(3.0) * t2676 * t188 + t2522 + t2523 + t2524 + t2525 + F::new(8.0) / F::new(3.0) * t2346 + t1412;
    t2680
}
