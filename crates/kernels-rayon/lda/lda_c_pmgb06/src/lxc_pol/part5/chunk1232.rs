//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1232/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1232(t12829: f64, t12832: f64, t20205: f64, t20207: f64, t20209: f64, t20210: f64, t20211: f64, t20213: f64, t20215: f64, t9759: f64, t9770: f64, t20219: f64, t20221: f64, t20222: f64, t20224: f64, t20226: f64, t20235: f64, t20238: f64, t20241: f64, t20243: f64, t20247: f64, t20250: f64, t20253: f64) -> (f64, f64) {
    let t21978 = -t20205 + t9759 - t20207 - t12829 - t12832 + t20209 + t20210 - t20211 - t9770 - t20213 + t20215;
    let t21979 = t20219 - t20221 - t20222 + t20224 + t20226 - t20235 - t20238 + t20241 - t20243 - t20247 - t20250 + t20253;
    (t21978, t21979)
}
