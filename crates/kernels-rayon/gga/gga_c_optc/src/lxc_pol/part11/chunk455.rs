//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 455/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk455(t115: f64, t2770: f64, t282: f64, t2769: f64, t1659: f64, t301: f64, t938: f64, t873: f64) -> (f64, f64, f64, f64) {
    let t2772 = t282 * t2770 * t115;
    let t2773 = t2769 * t2772;
    let t2778 = t1659 * t2772;
    let t2811 = t938 * t301;
    let t2812 = t2811 * t873;
    (t2773, t2778, t2811, t2812)
}
