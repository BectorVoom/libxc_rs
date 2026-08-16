//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 524/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk524(t1374: f64, t1379: f64, t1412: f64, t188: f64, t2346: f64, t2499: f64, t2503: f64, t2522: f64, t2523: f64, t2524: f64, t2525: f64, t2676: f64) -> f64 {
    let t2680 = t1374 + t1379 - t2499 - t2503 + 4.0_f64 / 3.0_f64 * t2676 * t188 + t2522 + t2523 + t2524 + t2525 + 8.0_f64 / 3.0_f64 * t2346 + t1412;
    t2680
}
