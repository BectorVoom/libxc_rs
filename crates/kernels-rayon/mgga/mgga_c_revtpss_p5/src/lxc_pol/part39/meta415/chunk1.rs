//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1505/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1505(t2184: f64, t5808: f64, t31328: f64, t575: f64, t1921: f64, t8283: f64, t1455: f64, t8389: f64, t116899: f64, t117090: f64, t117097: f64, t117099: f64, t117713: f64, t1456: f64, t18217: f64, t1914: f64, t2185: f64, t3: f64, t31127: f64, t31377: f64, t8284: f64) -> f64 {
    let t117781 = 2.0_f64 * t2184 * t5808;
    let t117783 = 2.0_f64 * t31328 * t575;
    let t117789 = 2.0_f64 * t8283 * t1921;
    let t117793 = 2.0_f64 * t1455 * t8389;
    let t117796 = t117713 * t3 * t575 + 2.0_f64 * t1456 * t31377 + t18217 * t2185 + t1914 * t31127 + 2.0_f64 * t5808 * t8284 + 2.0_f64 * t116899 + t117090 + t117097 + 2.0_f64 * t117099 + t117781 + t117783 + t117789 + t117793;
    t117796
}
