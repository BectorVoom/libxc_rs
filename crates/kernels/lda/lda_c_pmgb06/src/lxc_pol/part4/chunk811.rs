//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 811/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk811<F: Float>(t1983: F, t5499: F, t1382: F, t1444: F, t1920: F, t1925: F, t1972: F, t1981: F, t3451: F, t3454: F, t439: F, t493: F, t5458: F, t5464: F, t5467: F, t5471: F, t5474: F, t5477: F, t5483: F, t5487: F, t5494: F, t5497: F) -> (F, F) {
    let t5500 = t5499 * t1983;
    let t5504 = F::new(8.0) / F::new(45.0) * t1981 * t5458 + F::new(2.0) / F::new(27.0) * t1444 * t1920 + F::new(2.0) / F::new(27.0) * t493 * t5464 + t493 * t5467 / F::new(27.0) + F::new(8.0) / F::new(81.0) * t493 * t5471 - F::new(4.0) / F::new(27.0) * t1981 * t5474 - F::new(2.0) / F::new(45.0) * t493 * t5477 - F::new(2.0) / F::new(45.0) * t1972 * t1382 - F::new(2.0) / F::new(45.0) * t439 * t5483 - F::new(2.0) / F::new(45.0) * t493 * t5487 - F::new(2.0) / F::new(45.0) * t1444 * t1925 - F::new(2.0) / F::new(45.0) * t493 * t5494 - F::new(2.0) / F::new(405.0) * t5497 + F::new(2.0) / F::new(27.0) * t5500 + F::new(2.0) / F::new(135.0) * t3451 - t3454 / F::new(45.0);
    (t5500, t5504)
}
