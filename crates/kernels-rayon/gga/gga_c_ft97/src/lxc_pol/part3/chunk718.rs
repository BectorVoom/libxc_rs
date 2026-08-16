//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 718/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk718(t13794: f64, t1882: f64, t3696: f64, t3701: f64, t3951: f64, t761: f64, t1160: f64, t737: f64, t1144: f64, t8232: f64, t3991: f64, t3899: f64, t8392: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13795 = 4.0_f64 / 81.0_f64 * t13794;
    let t13809 = t1882 * t3696;
    let t13810 = 2.0_f64 / 27.0_f64 * t13809;
    let t13811 = t1882 * t3701;
    let t13812 = 4.0_f64 / 27.0_f64 * t13811;
    let t13830 = t3951 * t761;
    let t13839 = t737 * t1160;
    let t13872 = t8232 * t1144;
    let t13875 = 2.0_f64 / 9.0_f64 * t1882 * t3991;
    let t13884 = 2.0_f64 / 27.0_f64 * t8392 * t3899;
    (t13795, t13809, t13810, t13811, t13812, t13830, t13839, t13872, t13875, t13884)
}
