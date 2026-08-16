//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 900/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk900(t1808: f64, t1766: f64, t91: f64, t1586: f64, t22: f64, t36452: f64, t37991: f64, t96: f64, t1767: f64, t1775: f64, t8324: f64, t1554: f64) -> (f64, f64, f64, f64) {
    let t38447 = t1808 * t1808;
    let t38449 = t91 * t1766 * t38447;
    let t38456 = 1.0_f64 / t96 / t37991 / t22 / t1586 / t36452 / 96.0_f64;
    let t38457 = t1767 * t1767;
    let t38459 = t91 * t38456 * t38457;
    let t38461 = t1775 * t8324;
    let t38463 = t1554 * t1586;
    (t38449, t38459, t38461, t38463)
}
