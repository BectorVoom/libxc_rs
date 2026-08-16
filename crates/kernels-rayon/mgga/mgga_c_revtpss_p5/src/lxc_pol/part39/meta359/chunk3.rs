//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1243/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1243(t10868: f64, t241: f64, t820: f64, t14547: f64, t4364: f64, t4365: f64, t2724: f64, t2747: f64, t4450: f64, t14676: f64, t4366: f64, t10811: f64, t4452: f64) -> (f64, f64, f64, f64, f64) {
    let t14894 = t820 * t10868 * t241;
    let t14896 = t4364 * t4365 * t14547;
    let t14900 = t2747 * t4450 * t2724;
    let t14904 = t4364 * t14676 * t4366;
    let t14907 = t10811 * t4452;
    (t14894, t14896, t14900, t14904, t14907)
}
