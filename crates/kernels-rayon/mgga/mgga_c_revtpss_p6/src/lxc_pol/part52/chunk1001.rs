//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1001/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1001(t14224: f64, t26304: f64, t28845: f64, t7289: f64, t26356: f64, t26361: f64, t26363: f64, t27868: f64, t28826: f64, t28830: f64, t28838: f64, t28841: f64, t28846: f64, t28850: f64, t28853: f64, t7292: f64, t7295: f64, t7532: f64, t7917: f64, t8104: f64) -> f64 {
    let t28855 = t26304 * t14224;
    let t28858 = t7289 * t28845;
    let t28861 = -0.54878743191129263322e-2_f64 * t28826 + 0.4336814094102599731e0_f64 * t7295 * t28830 - 0.4336814094102599731e0_f64 * t7917 * t7532 - 0.4336814094102599731e0_f64 * t7292 * t8104 + 0.9757440539382783019e-2_f64 * t28838 + 0.8673628188205199462e0_f64 * t7295 * t28841 + 0.72280234901709995518e-2_f64 * t28846 + 0.4336814094102599731e0_f64 * t7295 * t28850 - 0.9757440539382783019e-2_f64 * t28853 + 0.4336814094102599731e0_f64 * t27868 * t28855 - 0.12851425765524037203e-1_f64 * t28858 - 0.54878743191129263322e-2_f64 * t26356 - t26361 + t26363;
    t28861
}
