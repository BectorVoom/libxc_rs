//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1004/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1004(t75340: f64, t75344: f64, t75347: f64, t75352: f64, t75360: f64, t75375: f64, t69294: f64, t75386: f64, t75388: f64, t75390: f64, t75393: f64, t75395: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78143 = 0.31062809106223861416e-1_f64 * t75340;
    let t78144 = 0.1814407727691612783e-2_f64 * t75344;
    let t78145 = 0.31752135234603223702e-2_f64 * t75347;
    let t78148 = 0.31752135234603223702e-2_f64 * t75352;
    let t78150 = 0.72324308034374009545e-3_f64 * t75360;
    let t78156 = 0.31062809106223861416e-2_f64 * t75375;
    let t78157 = 0.79828278012425390427e-1_f64 * t69294;
    let t78161 = 0.62125618212447722832e-2_f64 * t75386;
    let t78162 = 0.15531404553111930708e-1_f64 * t75388;
    let t78163 = 0.15531404553111930708e-1_f64 * t75390;
    let t78165 = 0.26609426004141796809e-1_f64 * t75393;
    let t78166 = 0.5987120850931904282e-1_f64 * t75395;
    (t78143, t78144, t78145, t78148, t78150, t78156, t78157, t78161, t78162, t78163, t78165, t78166)
}
