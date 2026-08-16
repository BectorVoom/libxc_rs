//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1907/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1907(t13869: f64, t7271: f64, t13878: f64, t25972: f64, t13967: f64, t26028: f64, t13937: f64, t13981: f64, t2689: f64, t27936: f64, t13857: f64, t94564: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98204 = t7271 * t13869;
    let t98206 = t25972 * t13878;
    let t98211 = t26028 * t13967;
    let t98213 = t26028 * t13937;
    let t98215 = t26028 * t13981;
    let t98218 = t2689 * t27936;
    let t98220 = t94564 * t13857;
    (t98204, t98206, t98211, t98213, t98215, t98218, t98220)
}
