//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1130/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1130(t278: f64, t41622: f64, t123: f64, t41670: f64, t805: f64, t41627: f64, t10327: f64, t1934: f64, t2347: f64, t274: f64, t2349: f64, t230: f64, t2417: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43707 = t41622 * t278;
    let t43712 = t123 / t805 / t41670;
    let t43715 = t41627 * t278;
    let t43726 = t10327 * t1934;
    let t43731 = t274 * t2347;
    let t43732 = t43731 * t2349;
    let t43736 = t230 * t2417;
    (t43707, t43712, t43715, t43726, t43732, t43736)
}
