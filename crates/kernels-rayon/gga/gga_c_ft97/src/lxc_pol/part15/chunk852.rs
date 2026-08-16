//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 852/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk852(t4977: f64, t679: f64, t2378: f64, t4939: f64, t237: f64, t39: f64, t13411: f64, t17818: f64, t7240: f64, t81: f64, t142: f64, t7367: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30683 = t679 * t4977;
    let t30688 = t2378 * t4939;
    let t30815 = t237 * t39;
    let t30852 = t13411 * t4939;
    let t30853 = t30852 * t17818;
    let t32075 = 1.0_f64 / t7240 / t81;
    let t32905 = 1.0_f64 / t7367 / t142;
    (t30683, t30688, t30815, t30852, t30853, t32075, t32905)
}
