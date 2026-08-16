//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1235/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1235(t18257: f64, t18259: f64, t20317: f64, t20319: f64, t20321: f64, t20323: f64, t20324: f64, t20325: f64, t20328: f64, t20330: f64, t20332: f64, t20334: f64, t20337: f64, t20338: f64, t20340: f64, t20343: f64, t20346: f64, t20353: f64, t20355: f64, t20359: f64, t20361: f64, t20364: f64, t20367: f64) -> (f64, f64) {
    let t21987 = -t20317 - t20319 - t20321 + t20323 + t20324 + t20325 + 2.0_f64 / 3.0_f64 * t18257 + 4.0_f64 / 3.0_f64 * t18259 - t20328 - t20330 - t20332;
    let t21988 = -t20334 - t20337 + t20338 + t20340 - t20343 + t20346 + t20353 - t20355 + t20359 + t20361 + t20364 + t20367;
    (t21987, t21988)
}
