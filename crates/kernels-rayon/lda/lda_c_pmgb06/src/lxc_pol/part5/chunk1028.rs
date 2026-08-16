//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1028/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1028(t1972: f64, t6131: f64, t6268: f64, t6536: f64, t1981: f64, t1982: f64, t6130: f64, t18020: f64, t835: f64, t1977: f64, t6134: f64, t19278: f64, t19280: f64, t19282: f64, t19284: f64, t19286: f64, t19289: f64, t19291: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19293 = t1972 * t6131 / 15.0_f64;
    let t19295 = 4.0_f64 / 15.0_f64 * t6268 * t6536;
    let t19298 = 2.0_f64 / 15.0_f64 * t1981 * t6130 * t1982;
    let t19300 = t18020 * t835 / 15.0_f64;
    let t19302 = t6134 * t1977 / 15.0_f64;
    let t19303 = t19278 + t19280 + t19282 - t19284 - t19286 - t19289 + t19291 + t19293 - t19295 - t19298 + t19300 + t19302;
    (t19293, t19295, t19298, t19300, t19302, t19303)
}
