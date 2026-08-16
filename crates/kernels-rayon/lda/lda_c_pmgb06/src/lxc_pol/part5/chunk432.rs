//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 432/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk432(t1545: f64, t432: f64, t824: f64, t1395: f64, t822: f64, t137: f64, t132: f64, t405: f64, t819: f64, t1619: f64, t1859: f64, t1864: f64, t473: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2039 = t1545 / 45.0_f64;
    let t2041 = t432 * t824 / 30.0_f64;
    let t2042 = t1395 * t822;
    let t2043 = t137 * t2042;
    let t2045 = t132 * t2043 / 30.0_f64;
    let t2052 = t405 * t819;
    let t2054 = t1619 * t1859;
    let t2057 = t473 * t1864;
    (t2039, t2041, t2042, t2043, t2045, t2052, t2054, t2057)
}
