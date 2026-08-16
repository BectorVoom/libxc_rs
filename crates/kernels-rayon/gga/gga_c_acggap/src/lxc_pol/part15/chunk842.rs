//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 842/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk842(t8185: f64, t8190: f64, t8192: f64, t8193: f64, t8195: f64, t8205: f64, t8209: f64, t8219: f64, t8220: f64, t8221: f64, t8232: f64, t8772: f64, t8829: f64, t9661: f64, t9664: f64, t9667: f64, t9671: f64, t9675: f64, t9677: f64) -> f64 {
    let t9922 = -t8185 + t8190 + t8192 + t8193 - t8195 - 0.916875e-1_f64 * t9661 + 0.4584375e-1_f64 * t9664 + 0.305625e-1_f64 * t9667 + 0.42874018118069736972e-2_f64 * t9671 - t8205 + t8209 + t8219 + t8220 - t8221 - 0.305625e-1_f64 * t8772 + 0.31448092289604152069e-3_f64 * t9675 - 0.16809375e0_f64 * t8829 + t8232 - 0.68598428988911579156e-2_f64 * t9677;
    t9922
}
