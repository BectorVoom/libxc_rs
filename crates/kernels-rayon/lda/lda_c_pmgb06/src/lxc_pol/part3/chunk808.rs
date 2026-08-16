//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 808/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk808(t856: f64, t97: f64, t1377: f64, t2342: f64, t27: f64, t545: f64, t2345: f64, t1366: f64, t2349: f64, t3019: f64, t3026: f64, t3028: f64, t5089: f64, t5093: f64, t5097: f64, t5101: f64, t5104: f64, t5107: f64, t5112: f64, t5114: f64) -> (f64, f64, f64, f64) {
    let t5649 = t856 * t97;
    let t5650 = t5649 * t1377;
    let t5652 = t2342 * t27;
    let t5654 = 0.21642082724729686_f64 * t5652 * t545;
    let t5655 = t2345 * t27;
    let t5656 = t5655 * t545;
    let t5658 = t2349 * t1366;
    let t5660 = -t5089 + t5093 + t5097 + 4.0_f64 / 3.0_f64 * t3019 + t3026 + 8.0_f64 / 3.0_f64 * t3028 + t5101 - t5104 - t5107 + t5112 - t5114 + 0.011181742741110338_f64 * t5650 + t5654 + 0.21642082724729686_f64 * t5656 + 0.07214027574909895_f64 * t5658;
    (t5649, t5652, t5655, t5660)
}
