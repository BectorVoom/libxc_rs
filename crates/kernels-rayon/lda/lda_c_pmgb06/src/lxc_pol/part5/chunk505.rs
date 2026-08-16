//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 505/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk505(t178: f64, t2563: f64, t831: f64, t844: f64, t1550: f64, t1557: f64, t1712: f64, t2027: f64, t2030: f64, t2032: f64, t2534: f64, t2535: f64, t2536: f64, t2557: f64) -> (f64, f64, f64) {
    let t2565 = t2563 * t178 / 30.0_f64;
    let t2567 = t831 * t844 / 15.0_f64;
    let t2568 = t2534 - t2535 - t2536 + 2.0_f64 / 3.0_f64 * t2027 + 0.12155555555555556_f64 * t2030 + 4.0_f64 / 9.0_f64 * t2032 - t1550 - t1557 + t1712 + t2557 + t2565 + t2567;
    (t2565, t2567, t2568)
}
