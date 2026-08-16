//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 916/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk916(t34: f64, t3615: f64, t109: f64, t1282: f64, t2247: f64, t2249: f64, t370: f64, t409: f64, t11404: f64, t69: f64, t11392: f64, t1773: f64, t2262: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11470 = t34 * t3615;
    let t11475 = t109 * t1282;
    let t11485 = t2247 * t409 * t370 * t2249;
    let t11519 = t69 * t11404;
    let t11521 = t69 * t11392;
    let t11567 = t1773 * t2262;
    (t11470, t11475, t11485, t11519, t11521, t11567)
}
