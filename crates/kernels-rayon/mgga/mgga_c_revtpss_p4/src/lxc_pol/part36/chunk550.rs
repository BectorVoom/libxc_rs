//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 550/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk550(t1179: f64, t1749: f64, t1756: f64, t3523: f64, t300: f64, t3495: f64, t1208: f64, t1769: f64) -> (f64, f64, f64, f64, f64) {
    let t5158 = t1749 * t1179;
    let t5184 = t1756 * t3523;
    let t5192 = t300 * t1749;
    let t5197 = t3495 * t1756;
    let t5219 = t1769 * t1208;
    (t5158, t5184, t5192, t5197, t5219)
}
