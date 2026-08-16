//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1115/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1115(t13241: f64, t1902: f64, t3213: f64, t1447: f64, t5494: f64, t1387: f64, t5187: f64, t5487: f64, t1423: f64, t5483: f64, t10079: f64, t13231: f64, t13233: f64, t13236: f64, t13238: f64, t13240: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13242 = 4.0_f64 / 45.0_f64 * t13241;
    let t13243 = t3213 * t1902;
    let t13244 = 2.0_f64 / 81.0_f64 * t13243;
    let t13245 = t1447 * t5494;
    let t13246 = 4.0_f64 / 45.0_f64 * t13245;
    let t13248 = 2.0_f64 / 15.0_f64 * t5187 * t1387;
    let t13249 = t1447 * t5487;
    let t13250 = 4.0_f64 / 45.0_f64 * t13249;
    let t13251 = t1423 * t5483;
    let t13252 = 4.0_f64 / 45.0_f64 * t13251;
    let t13254 = t13231 + t13233 + t13236 + t13238 + t13240 + t13242 - t13244 - t13246 - t13248 - t13250 - t13252 - 8.0_f64 / 135.0_f64 * t10079;
    (t13242, t13244, t13246, t13248, t13250, t13252, t13254)
}
