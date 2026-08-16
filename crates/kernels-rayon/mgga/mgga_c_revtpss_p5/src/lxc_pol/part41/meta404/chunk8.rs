//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1407/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1407(t10275: f64, t10278: f64, t10284: f64, t10287: f64, t10295: f64, t13261: f64, t13262: f64, t13263: f64, t13264: f64, t13265: f64, t13266: f64, t5812: f64, t602: f64) -> (f64, f64) {
    let t21661 = t13261 - t13262 - t10275 + t10278 + t13263 - t13264 - t10284 + t10287 + t13265 - t13266 - t10295;
    let t21663 = t5812 * t602;
    (t21661, t21663)
}
