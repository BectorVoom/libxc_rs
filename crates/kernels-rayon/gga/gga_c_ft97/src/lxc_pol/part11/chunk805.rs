//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 805/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk805(t10894: f64, t898: f64, t900: f64, t231: f64, t8608: f64, t893: f64, t2937: f64, t325: f64, t2939: f64, t904: f64, t2938: f64, t2951: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10896 = t898 * t900 * t10894;
    let t10900 = t231 * t893 * t8608;
    let t10904 = 1.0_f64 / t2937 / t325;
    let t10905 = t2939 * t904;
    let t10907 = t898 * t10904 * t10905;
    let t10912 = t898 * t2938 * t904 * t2951;
    (t10896, t10900, t10904, t10905, t10907, t10912)
}
