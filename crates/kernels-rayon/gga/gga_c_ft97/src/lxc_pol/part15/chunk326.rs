//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 326/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk326(t1984: f64, t2: f64, t1956: f64, t161: f64, t1637: f64, t89: f64, t159: f64, t603: f64) -> (f64, f64, f64, f64, f64) {
    let t2112 = t1984 * t2;
    let t2124 = 4.0_f64 / 27.0_f64 * t1956;
    let t2149 = 4.0_f64 / 9.0_f64 * t1956;
    let t2164 = 4.0_f64 / 27.0_f64 * t89 * t1637 * t161;
    let t2178 = 1.0_f64 / t603 / t159;
    (t2112, t2124, t2149, t2164, t2178)
}
