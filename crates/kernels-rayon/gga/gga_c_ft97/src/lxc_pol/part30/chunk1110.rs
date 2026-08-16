//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1110/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1110(t143163: f64, t152799: f64, t33820: f64, t35981: f64, t681: f64, t89: f64, t35993: f64, t25162: f64, t35860: f64, t10683: f64, t28501: f64, t6317: f64, t6318: f64) -> (f64, f64, f64, f64, f64) {
    let t152899 = t33820 * t143163 * t152799;
    let t152902 = t89 * t681 * t35981;
    let t152905 = t89 * t681 * t35993;
    let t152907 = t25162 * t35860;
    let t152913 = t6317 * t10683 * t6318 * t28501;
    (t152899, t152902, t152905, t152907, t152913)
}
