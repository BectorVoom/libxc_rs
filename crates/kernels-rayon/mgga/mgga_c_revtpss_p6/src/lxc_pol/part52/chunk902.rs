//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 902/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk902(t27291: f64, t7076: f64, t1959: f64, t25297: f64, t25303: f64, t25307: f64, t25311: f64, t25333: f64, t25337: f64, t25340: f64, t25353: f64, t25356: f64, t25383: f64, t27275: f64, t27280: f64, t27287: f64, t7070: f64, t7775: f64) -> (f64, f64) {
    let t27292 = t7076 * t27291;
    let t27297 = 0.72280234901709995518e-2_f64 * t25297 + t25303 - t25307 + 0.72280234901709995518e-2_f64 * t25311 - 0.4336814094102599731e0_f64 * t27275 * t1959 + t25333 - 0.12851425765524037203e-1_f64 * t27280 - t25337 - 0.54878743191129263322e-2_f64 * t25340 + 0.4336814094102599731e0_f64 * t25383 * t7775 + 0.4336814094102599731e0_f64 * t7070 * t27287 + 0.4336814094102599731e0_f64 * t7070 * t27292 + 0.54878743191129263322e-2_f64 * t25353 + 0.9757440539382783019e-2_f64 * t25356;
    (t27292, t27297)
}
