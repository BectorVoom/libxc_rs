//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1084/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1084(t1: f64, t1385: f64, t2010: f64, t6773: f64, t10699: f64, t20009: f64, t20012: f64, t20014: f64, t20017: f64, t20021: f64, t20025: f64, t20028: f64, t20031: f64, t20035: f64) -> (f64, f64) {
    let t20039 = 2.0_f64 / 15.0_f64 * t2010 * t1385 * t6773 * t1;
    let t20040 = 4.0_f64 * t10699 - t20009 - t20012 - t20014 - t20017 - t20021 + t20025 - t20028 + t20031 - t20035 - t20039;
    (t20039, t20040)
}
