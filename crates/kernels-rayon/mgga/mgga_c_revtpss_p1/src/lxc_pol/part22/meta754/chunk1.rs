//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2830/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2830(t3181: f64, t675: f64, t1063: f64, t247: f64, t2853: f64, t283: f64, t2852: f64, t1025: f64, t3218: f64, t371: f64, t676: f64, t11144: f64, t3252: f64) -> (f64, f64, f64, f64, f64) {
    let t42447 = t675 * t3181;
    let t42450 = t1063 * t247 * t42447 * t2853;
    let t42471 = 1.0_f64 / t283 / t2852;
    let t42481 = t1025 * t371 * t676 * t3218;
    let t42518 = t3252 * t11144;
    (t42447, t42450, t42471, t42481, t42518)
}
