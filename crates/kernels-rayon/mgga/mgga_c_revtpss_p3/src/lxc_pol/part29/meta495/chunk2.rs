//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1803/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1803(t25921: f64, t26232: f64, t26235: f64, t26238: f64, t26251: f64, t26253: f64, t26263: f64, t26266: f64, t26268: f64, t26272: f64, t28781: f64, t28783: f64, t28792: f64, t28796: f64, t7295: f64, t8100: f64) -> f64 {
    let t28799 = -0.72280234901709995518e-2_f64 * t26232 + 0.25702851531048074406e-1_f64 * t28781 - 0.14456046980341999104e-1_f64 * t28783 - 0.14456046980341999104e-1_f64 * t26235 - t26238 + t26251 + 0.9757440539382783019e-2_f64 * t26253 - t26263 - 0.9757440539382783019e-2_f64 * t26266 + 0.4336814094102599731e0_f64 * t25921 * t8100 + 0.4336814094102599731e0_f64 * t7295 * t28792 + 0.12851425765524037203e-1_f64 * t26268 - 0.12851425765524037203e-1_f64 * t28796 + 0.72280234901709995518e-2_f64 * t26272;
    t28799
}
