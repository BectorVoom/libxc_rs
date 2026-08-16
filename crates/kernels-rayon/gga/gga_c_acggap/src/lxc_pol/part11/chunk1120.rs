//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1120/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1120(t31168: f64, t13299: f64, t31057: f64, t35288: f64, t4643: f64, t7486: f64, t2095: f64, t1427: f64, t31491: f64, t7381: f64, t1345: f64, t1983: f64, t7380: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35373 = 0.14291339372689912324e-2_f64 * t31168;
    let t35379 = t31057 * t13299 * t35288;
    let t35380 = 0.31448092289604152068e-3_f64 * t35379;
    let t35383 = t4643 * t7486;
    let t35384 = t2095 * t35383;
    let t35385 = 0.305625e-1_f64 * t35384;
    let t35387 = t31491 * t7381 * t1427;
    let t35388 = t35387 / 8.0_f64;
    let t35390 = t7380 * t1983 * t1345;
    (t35373, t35380, t35383, t35385, t35388, t35390)
}
