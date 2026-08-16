//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 819/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk819(t2345: f64, t26: f64, t2347: f64, t743: f64, t666: f64, t2360: f64, t2567: f64, t668: f64, t2486: f64, t754: f64, t2372: f64, t255: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13682 = t26 * t2345;
    let t13683 = t743 * t2347;
    let t13688 = t26 * t666;
    let t13689 = t743 * t2360;
    let t13857 = t2567 * t668;
    let t13879 = t2486 * t754;
    let t13885 = t2372 * t255;
    (t13682, t13683, t13688, t13689, t13857, t13879, t13885)
}
