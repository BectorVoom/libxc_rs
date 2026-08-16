//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1500/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1500(t283: f64, t2852: f64, t66: f64, t11951: f64, t3211: f64, t1025: f64, t3218: f64, t371: f64, t676: f64, t11804: f64, t11921: f64, t247: f64, t4837: f64) -> (f64, f64, f64, f64) {
    let t42471 = 1.0_f64 / t283 / t2852;
    let t42472 = t66 * t42471;
    let t42477 = t3211 * t11951;
    let t42481 = t1025 * t371 * t676 * t3218;
    let t42487 = t4837 * t247 * t11921 * t11804;
    (t42472, t42477, t42481, t42487)
}
