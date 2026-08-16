//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 361/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk361(t1749: f64, t448: f64, t1182: f64, t1185: f64, t1717: f64, t1724: f64, t1727: f64, t1730: f64) -> (f64, f64) {
    let t1750 = t1749 * t448;
    let t1756 = 0.258925e1_f64 * t1724 - t1182 + 0.301925e0_f64 * t1717 + 0.16504875e0_f64 * t1727 - t1185 + 0.82785e-1_f64 * t1730;
    (t1750, t1756)
}
