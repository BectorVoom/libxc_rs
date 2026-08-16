//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1368/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1368(t16712: f64, t12256: f64, t1469: f64, t2251: f64, t12305: f64, t128: f64) -> (f64, f64, f64) {
    let t16713 = 0.9877777777777777778e-2_f64 * t16712;
    let t16714 = t12256 * t1469;
    let t16715 = t16714 * t2251;
    let t16716 = t12305 * t16715;
    let t16717 = t128 * t16716;
    (t16713, t16715, t16717)
}
