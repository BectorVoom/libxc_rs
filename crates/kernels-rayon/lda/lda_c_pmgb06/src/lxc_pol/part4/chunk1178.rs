//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1178/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1178(t1848: f64, t1933: f64, t1420: f64, t6491: f64, t12772: f64, t1893: f64, t439: f64, t5290: f64, t5482: f64, t2010: f64, t5294: f64, t15468: f64, t15469: f64, t15470: f64, t15471: f64, t15473: f64, t15474: f64, t15475: f64, t15476: f64, t15480: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15481 = t1848 * t1933;
    let t15482 = 4.0_f64 / 45.0_f64 * t15481;
    let t15484 = 4.0_f64 / 45.0_f64 * t1420 * t6491;
    let t15487 = 4.0_f64 / 45.0_f64 * t439 * t12772 * t1893;
    let t15490 = 2.0_f64 / 45.0_f64 * t439 * t5482 * t5290;
    let t15493 = 8.0_f64 / 45.0_f64 * t2010 * t5482 * t5294;
    let t15494 = -t15468 + t15469 - t15470 - t15471 - t15473 - t15474 - t15475 - t15476 - t15480 + t15482 - t15484 - t15487 - t15490 - t15493;
    (t15482, t15484, t15487, t15490, t15493, t15494)
}
