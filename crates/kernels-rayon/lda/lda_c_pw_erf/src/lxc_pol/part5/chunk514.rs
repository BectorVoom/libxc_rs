//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 514/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk514(t2124: f64, t2128: f64, t2135: f64, t2138: f64, t2141: f64, t2144: f64, t1409: f64, t1412: f64, t1429: f64, t1435: f64, t1439: f64, t1521: f64, t1531: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2573 = 8.0_f64 / 45.0_f64 * t2124;
    let t2574 = 16.0_f64 / 45.0_f64 * t2128;
    let t2575 = 8.0_f64 / 45.0_f64 * t2135;
    let t2576 = 16.0_f64 / 45.0_f64 * t2138;
    let t2577 = 16.0_f64 / 135.0_f64 * t2141;
    let t2578 = 16.0_f64 / 135.0_f64 * t2144;
    let t2579 = t1409 - t1412 + t1429 + t1435 + t1439 - t1521 - t1531 - t2573 + t2574 - t2575 + t2576 + t2577 + t2578;
    (t2573, t2574, t2575, t2576, t2577, t2578, t2579)
}
