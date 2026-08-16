//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1100/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1100(t16605: f64, t436: f64, t7465: f64, t1928: f64, t2592: f64, t12832: f64, t20209: f64, t20210: f64, t20211: f64, t20213: f64, t20215: f64, t20219: f64, t20221: f64, t9770: f64) -> (f64, f64, f64, f64) {
    let t20222 = t16605 / 15.0_f64;
    let t20223 = t7465 * t436;
    let t20224 = t20223 / 45.0_f64;
    let t20225 = t2592 * t1928;
    let t20226 = t20225 / 15.0_f64;
    let t20227 = -t12832 + t20209 + t20210 - t20211 - t9770 - t20213 + t20215 + t20219 - t20221 - t20222 + t20224 + t20226;
    (t20222, t20224, t20226, t20227)
}
