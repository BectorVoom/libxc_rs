//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1238/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1238(t16305: f64, t2470: f64, t3223: f64, t1447: f64, t6120: f64, t439: f64, t4766: f64, t6550: f64, t2477: f64, t3213: f64, t10687: f64, t10690: f64, t16293: f64, t16295: f64, t16297: f64, t16299: f64, t16300: f64, t16301: f64, t16302: f64, t16303: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16306 = t16305 / 135.0_f64;
    let t16307 = t3223 * t2470;
    let t16308 = 2.0_f64 / 243.0_f64 * t16307;
    let t16309 = t1447 * t6120;
    let t16310 = 8.0_f64 / 45.0_f64 * t16309;
    let t16313 = 2.0_f64 / 5.0_f64 * t439 * t6550 * t4766;
    let t16314 = t3213 * t2477;
    let t16315 = 4.0_f64 / 405.0_f64 * t16314;
    let t16316 = -t16293 - t16295 - t16297 - t16299 + t16300 + t16301 + t16302 + t16303 - t16306 - t10687 + t10690 - t16308 + t16310 - t16313 - t16315;
    (t16306, t16308, t16310, t16313, t16315, t16316)
}
