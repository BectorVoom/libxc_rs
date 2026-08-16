//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 974/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk974(t231: f64, t7997: f64, t836: f64, t7076: f64, t1558: f64, t7398: f64, t1580: f64, t7384: f64, t689: f64, t213: f64, t25383: f64, t26498: f64, t26500: f64, t26547: f64, t28361: f64, t28366: f64, t28369: f64, t28371: f64, t28374: f64, t7067: f64, t7070: f64, t8012: f64, t8016: f64, t887: f64) -> f64 {
    let t28377 = t7997 * t836 * t231;
    let t28378 = t7076 * t28377;
    let t28384 = t7398 * t1558 * t231;
    let t28385 = t7076 * t28384;
    let t28390 = t7384 * t1580;
    let t28391 = t689 * t28390;
    let t28394 = t213 * t7997;
    let t28397 = 0.72280234901709995518e-2_f64 * t28361 - 0.65854491829355115987e0_f64 * t26547 * t1580 - 0.9757440539382783019e-2_f64 * t26498 - 0.12851425765524037203e-1_f64 * t28366 - 0.72280234901709995518e-2_f64 * t28369 + 0.12851425765524037203e-1_f64 * t28371 + 0.9757440539382783019e-2_f64 * t28374 + 0.4336814094102599731e0_f64 * t7070 * t28378 + 0.4336814094102599731e0_f64 * t25383 * t8012 + 0.4336814094102599731e0_f64 * t7070 * t28385 - 0.4336814094102599731e0_f64 * t7067 * t8016 + 0.54878743191129263322e-2_f64 * t28391 - 0.72280234901709995518e-2_f64 * t26500 - 0.65854491829355115987e0_f64 * t28394 * t887;
    t28397
}
