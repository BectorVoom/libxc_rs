//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 777/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk777(t28341: f64, t6637: f64, t23035: f64, t1484: f64, t25319: f64, t6552: f64, t1510: f64, t25255: f64, t1499: f64, t23014: f64, t23032: f64, t25246: f64, t25259: f64, t28323: f64, t28331: f64, t28335: f64, t28339: f64, t4166: f64, t7533: f64, t7535: f64, t812: f64) -> (f64, f64, f64) {
    let t28342 = t6637 * t28341;
    let t28343 = t23035 * t28342;
    let t28345 = t25319 * t1484;
    let t28346 = t6637 * t28345;
    let t28347 = t6552 * t28346;
    let t28351 = t25255 * t1510;
    let t28354 = -0.82246703342411321825e-2_f64 * t28323 + 0.82246703342411321824e-2_f64 * t25246 + 2.0_f64 * t1499 * t7535 - 0.82246703342411321824e-2_f64 * t25259 - 0.16449340668482264365e-1_f64 * t28331 + t23014 + t23032 + 0.82246703342411321825e-2_f64 * t28335 + 0.3289868133696452873e-1_f64 * t28339 + 0.49348022005446793095e-1_f64 * t28343 - 0.3289868133696452873e-1_f64 * t28347 - 2.0_f64 * t4166 * t7533 - 2.0_f64 * t812 * t28351;
    (t28343, t28347, t28354)
}
