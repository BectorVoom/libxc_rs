//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 116/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk116(t676: f64, t713: f64, t27: f64, t89: f64, t664: f64, t672: f64, t661: f64, t259: f64, t681: f64, t241: f64, t683: f64) -> (f64, f64, f64, f64, f64) {
    let t714 = t676 * t713;
    let t716 = t89 * t27 * t714;
    let t718 = -t664 - t672 / 18.0_f64 - t716 / 6.0_f64;
    let t719 = t661 * t718;
    let t723 = t89 * t681 * t259 / 9.0_f64;
    let t724 = t683 * t241;
    (t714, t716, t719, t723, t724)
}
