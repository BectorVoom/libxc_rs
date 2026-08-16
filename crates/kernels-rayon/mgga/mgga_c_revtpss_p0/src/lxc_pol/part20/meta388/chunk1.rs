//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1419/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1419(t11342: f64, t698: f64, t11821: f64, t240: f64, t2851: f64, t39443: f64, t141: f64, t39457: f64, t905: f64, t930: f64, t25273: f64, t268: f64, t271: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41292 = t698 * t11342;
    let t41294 = t240 * t11821;
    let t41295 = t2851 * t2851;
    let t41296 = 1.0_f64 / t41295;
    let t41297 = t41296 * t39443;
    let t41299 = t141 * t41294 * t41297;
    let t41301 = t905 * t39457;
    let t41303 = t141 * t930 * t41301;
    let t41306 = t268 * t25273 * t271;
    (t41292, t41296, t41297, t41299, t41301, t41303, t41306)
}
