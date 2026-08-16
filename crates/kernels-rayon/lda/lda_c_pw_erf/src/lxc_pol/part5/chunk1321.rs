//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1321/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1321(t12714: f64, t21364: f64, t21365: f64, t21366: f64, t21378: f64, t21381: f64, t21384: f64, t21385: f64, t21387: f64, t21388: f64, t21389: f64, t21390: f64, t21392: f64) -> f64 {
    let t23242 = t21364 + t21365 + t21366 + 0.0033101111111111113_f64 * t12714 - t21378 - t21381 - t21384 - t21385 + t21387 + t21388 + t21389 + t21390 - t21392;
    t23242
}
