//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 797/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk797(t139: f64, t527: f64, t52: f64, t538: f64, t7182: f64, t1995: f64, t32311: f64, t7335: f64, t71: f64, t420: f64, t7195: f64, t135: f64, t32318: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32815 = t527 * t139;
    let t32817 = t52 * t7182 * t538;
    let t32822 = t1995 * t139;
    let t32836 = 0.26675734978222673832e-1_f64 * t7335 * t32311;
    let t32837 = t71 * t538;
    let t32838 = t420 * t32837;
    let t32839 = t7195 * t32838;
    let t32848 = t32318 * t135;
    (t32815, t32817, t32822, t32836, t32837, t32839, t32848)
}
