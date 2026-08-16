//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 378/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk378(t1352: f64, t1371: f64, t1356: f64, t589: f64, t1360: f64, t1346: f64, t1347: f64, t1354: f64, t1358: f64, t1362: f64, t1366: f64, t1367: f64, t25: f64) -> (f64, f64, f64, f64) {
    let t1372 = t1371 * t1352;
    let t1375 = t589 * t1356;
    let t1378 = t589 * t1360;
    let t1381 = t1346 + 0.023994444444444443_f64 * t1347 - 0.023994444444444443_f64 * t1354 + 0.07198333333333333_f64 * t1358 - 0.035991666666666665_f64 * t1362 + t1366 + 0.008888888888888889_f64 * t1367 - 0.0022222222222222222_f64 * t25 * t1372 + 0.013333333333333334_f64 * t25 * t1375 - 0.006666666666666667_f64 * t25 * t1378;
    (t1372, t1375, t1378, t1381)
}
