//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1054/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1054(t75352: f64, t75360: f64, t75375: f64, t69294: f64, t75386: f64, t75388: f64, t75390: f64, t75356: f64, t75362: f64, t75364: f64, t75367: f64, t75369: f64, t75371: f64, t75378: f64, t75380: f64, t75383: f64) -> f64 {
    let t78148 = 0.31752135234603223702e-2_f64 * t75352;
    let t78150 = 0.72324308034374009545e-3_f64 * t75360;
    let t78156 = 0.31062809106223861416e-2_f64 * t75375;
    let t78157 = 0.79828278012425390427e-1_f64 * t69294;
    let t78161 = 0.62125618212447722832e-2_f64 * t75386;
    let t78162 = 0.15531404553111930708e-1_f64 * t75388;
    let t78163 = 0.15531404553111930708e-1_f64 * t75390;
    let t78164 = t78148 - 0.50803416375365157923e-2_f64 * t75356 + t78150 + 0.24192103035888170439e-2_f64 * t75362 - 0.33868944250243438615e-2_f64 * t75364 - 0.68186654135613354322e-2_f64 * t75367 - 0.68186654135613354322e-2_f64 * t75369 + 0.13637330827122670864e-1_f64 * t75371 + t78156 + t78157 + 0.27274661654245341729e-1_f64 * t75378 + 0.27274661654245341728e-1_f64 * t75380 - 0.6818665413561335432e-1_f64 * t75383 + t78161 - t78162 - t78163;
    t78164
}
