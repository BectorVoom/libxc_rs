//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 771/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk771(t10284: f64, t10430: f64, t788: f64, t2801: f64, t684: f64, t870: f64, t2881: f64, t2770: f64, t863: f64, t2877: f64, t848: f64, t2884: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10431 = t10284 + t10430;
    let t10432 = t788 * t10431;
    let t10439 = t870 * t2801 * t684;
    let t10440 = t2881 * t10439;
    let t10443 = t2770 * t863;
    let t10444 = t10443 * t2877;
    let t10447 = t848 * t863;
    let t10448 = t10447 * t2884;
    (t10431, t10432, t10439, t10440, t10443, t10444, t10447, t10448)
}
