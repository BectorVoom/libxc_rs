//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1023/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1023(t15467: f64, t11777: f64, t15472: f64, t6688: f64, t853: f64, t1447: f64, t7671: f64, t1423: f64, t7667: f64, t7640: f64, t19224: f64, t19227: f64, t19231: f64, t19233: f64, t19236: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19237 = 2.0_f64 / 45.0_f64 * t15467;
    let t19238 = 4.0_f64 / 135.0_f64 * t11777;
    let t19239 = 2.0_f64 / 15.0_f64 * t15472;
    let t19241 = t6688 * t853 / 10.0_f64;
    let t19242 = t1447 * t7671;
    let t19243 = 4.0_f64 / 45.0_f64 * t19242;
    let t19244 = t1423 * t7667;
    let t19245 = 4.0_f64 / 45.0_f64 * t19244;
    let t19246 = t1447 * t7640;
    let t19247 = 2.0_f64 / 135.0_f64 * t19246;
    let t19248 = -t19224 - t19227 - t19231 - t19233 + t19236 - t19237 + t19238 - t19239 - t19241 + t19243 + t19245 + t19247;
    (t19237, t19238, t19239, t19241, t19243, t19245, t19247, t19248)
}
