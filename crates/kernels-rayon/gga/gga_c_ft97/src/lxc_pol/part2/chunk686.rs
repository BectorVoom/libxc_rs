//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 686/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk686(t313: f64, t89: f64, t9555: f64, t1882: f64, t2811: f64, t2807: f64, t295: f64, t9568: f64, t2803: f64, t8232: f64, t842: f64, t10397: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10749 = 28.0_f64 / 81.0_f64 * t89 * t9555 * t313;
    let t10750 = t1882 * t2811;
    let t10752 = t1882 * t2807;
    let t10758 = t9568 * t295;
    let t10771 = t1882 * t2803;
    let t10773 = t8232 * t842;
    let t10797 = 28.0_f64 / 27.0_f64 * t10397;
    (t10749, t10750, t10752, t10758, t10771, t10773, t10797)
}
