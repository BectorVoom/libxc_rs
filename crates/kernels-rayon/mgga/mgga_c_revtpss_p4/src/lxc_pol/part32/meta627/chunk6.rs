//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2008/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2008(t212: f64, t30379: f64, t689: f64, t780: f64, t105936: f64, t95537: f64, t213: f64, t102947: f64, t102953: f64, t102956: f64, t102964: f64, t102969: f64, t103424: f64, t25317: f64, t25391: f64, t27312: f64, t6048: f64, t7070: f64, t7398: f64, t887: f64, t95542: f64, t95548: f64, t95551: f64, t95562: f64) -> f64 {
    let t110245 = t689 * t212 * t30379 * t780;
    let t110247 = t95537 * t105936;
    let t110256 = t213 * t30379;
    let t110261 = -0.54878743191129263322e-2_f64 * t110245 - t102947 - t95542 - 0.51405703062096148813e-1_f64 * t110247 - 0.26020884564615598386e1_f64 * t7070 * t25317 * t7398 * t6048 - t95548 - 0.17347256376410398924e1_f64 * t25391 * t103424 * t27312 - t102953 + t102956 - 0.65854491829355115987e0_f64 * t110256 * t887 - 0.96373646535613327357e-2_f64 * t95551 + t102964 - 0.65049603595885220126e-3_f64 * t95562 - t102969;
    t110261
}
