//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 723/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk723(t13143: f64, t13151: f64, t13679: f64, t13681: f64, t13693: f64, t13695: f64, t13697: f64, t13700: f64, t13703: f64, t13704: f64, t13898: f64, t13899: f64) -> f64 {
    let t14406 = t13679 + t13681 - t13693 - t13695 + t13697 - t13898 + t13899 + t13700 + t13703 - 0.44688112439813033337e-1_f64 * t13704 + 0.63904876589867916127e-1_f64 * t13143 - 0.63904876589867916127e-1_f64 * t13151;
    t14406
}
