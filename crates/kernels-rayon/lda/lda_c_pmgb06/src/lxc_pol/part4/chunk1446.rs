//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1446/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1446(t1234: f64, t1316: f64, t1317: f64, t15136: f64, t15168: f64, t18044: f64, t18056: f64, t18087: f64, t18095: f64, t18127: f64, t18151: f64, t18410: f64, t18440: f64, t2180: f64, t2236: f64, t2255: f64, t2311: f64, t2733: f64, t295: f64, t342: f64, t4006: f64, t4231: f64, t4232: f64, t4233: f64, t5583: f64, t5883: f64, t61: f64, t7089: f64, t787: f64, t790: f64, t8177: f64, t8180: f64, t8184: f64, t8187: f64, t8189: f64, t8206: f64, t8208: f64, t8211: f64) -> f64 {
    let t18444 = -t8177 - 9.138438188948293e-06_f64 * t8180 - t8184 + 0.039914113367515366_f64 * t8187 + 0.11974234010254609_f64 * t8189 - 0.01197423401025461_f64 * t8206 + 0.3902713307045947_f64 * t8208 + t8211 - 6.0_f64 * t5583 * t4232 * t2236 * t342 - 6.0_f64 * t5583 * t15136 * t4233 + (t15168 + t18044 + t18056 + t18087) * t61 + 6.0_f64 * t1316 * t2733 * t4006 - 3.0_f64 * t4231 * t18095 + 6.0_f64 * t1316 * t790 * t2255 * t342 + 6.0_f64 * t1316 * t7089 * t1317 + 6.0_f64 * t1316 * t5883 * t2311 + 6.0_f64 * t2180 * t790 * t787 * t1234 + (t18127 + t18151 + t18410 + t18440) * t295;
    t18444
}
