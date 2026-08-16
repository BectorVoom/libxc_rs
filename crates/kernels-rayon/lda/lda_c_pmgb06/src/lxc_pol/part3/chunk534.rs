//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 534/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk534(t2341: f64, t2352: f64, t2354: f64, t2358: f64, t117: f64, t118: f64, t123: f64, t125: f64, t1328: f64, t1330: f64, t1333: f64, t1337: f64, t1341: f64, t1345: f64, t1349: f64, t1352: f64, t1356: f64, t1360: f64, t1363: f64, t1799: f64, t2323: f64, t2327: f64, t2331: f64, t2338: f64) -> (f64, f64) {
    let t2360 = t2341 + t2352 + t2354 + t2358;
    let t2365 = -t1328 + 0.031505407223141116_f64 * t1330 + t1333 + t1337 + 0.031505407223141116_f64 * t2323 - 0.031505407223141116_f64 * t1799 * t118 - 0.031505407223141116_f64 * t2327 - 0.001975389032890948_f64 * t2331 - 0.031505407223141116_f64 * t1341 - t1349 - t1352 - 0.001975389032890948_f64 * t1345 - t1356 - t1360 + 0.008980675507690957_f64 * t1363 + 0.008980675507690957_f64 * t2338 - 0.005388405304614574_f64 * t123 * t125 * t2360 * t117;
    (t2360, t2365)
}
