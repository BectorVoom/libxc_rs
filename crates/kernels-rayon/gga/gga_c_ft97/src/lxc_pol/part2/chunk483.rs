//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 483/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk483(t2801: f64, t871: f64, t296: f64, t824: f64, t840: f64, t882: f64, t2739: f64, t319: f64, t2399: f64, t313: f64, t89: f64, t1882: f64, t842: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2802 = t871 * t2801;
    let t2803 = t296 * t2802;
    let t2807 = t840 * t882 * t824;
    let t2811 = t840 * t319 * t2739;
    let t2816 = 4.0_f64 / 27.0_f64 * t89 * t2399 * t313;
    let t2817 = t1882 * t842;
    (t2802, t2803, t2807, t2811, t2816, t2817)
}
