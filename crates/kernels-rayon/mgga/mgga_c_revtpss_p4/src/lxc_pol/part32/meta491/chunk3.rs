//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1750/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1750(t2067: f64, t25391: f64, t26541: f64, t26545: f64, t26557: f64, t26558: f64, t26561: f64, t26564: f64, t26578: f64, t27199: f64, t27275: f64, t27353: f64, t28426: f64, t28434: f64, t28436: f64, t28439: f64, t28442: f64, t28449: f64, t7415: f64) -> f64 {
    let t28453 = -0.8673628188205199462e0_f64 * t27353 * t28426 - 0.14456046980341999104e-1_f64 * t26541 + 0.72280234901709995518e-2_f64 * t26545 - t26557 - 0.4336814094102599731e0_f64 * t27275 * t2067 - 0.12851425765524037203e-1_f64 * t26558 - 0.9757440539382783019e-2_f64 * t28434 - 0.8673628188205199462e0_f64 * t25391 * t28436 + 0.4336814094102599731e0_f64 * t27353 * t28439 - 0.8673628188205199462e0_f64 * t25391 * t28442 + 0.8673628188205199462e0_f64 * t27199 * t7415 - 0.54878743191129263322e-2_f64 * t28449 + 0.54878743191129263322e-2_f64 * t26561 + 0.9757440539382783019e-2_f64 * t26564 + t26578;
    t28453
}
