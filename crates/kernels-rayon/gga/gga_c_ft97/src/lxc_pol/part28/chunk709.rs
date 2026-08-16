//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 709/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk709(t12703: f64, t27096: f64, t27064: f64, t23463: f64, t925: f64, t2210: f64, t23470: f64, t3420: f64, t379: f64, t6708: f64, t13220: f64, t6699: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27239 = t12703 * t27096;
    let t27242 = t12703 * t27064;
    let t27245 = t23463 * t925;
    let t27246 = t2210 * t27245;
    let t27249 = t23470 * t3420;
    let t27252 = t6708 * t379;
    let t27253 = t13220 * t27252;
    let t27256 = t6699 * t379;
    (t27239, t27242, t27245, t27246, t27249, t27252, t27253, t27256)
}
