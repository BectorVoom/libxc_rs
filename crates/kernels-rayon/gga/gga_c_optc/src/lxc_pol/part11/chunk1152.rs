//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1152/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1152(t2640: f64, t41521: f64, t4947: f64, t10894: f64, t16968: f64, t17079: f64, t907: f64, t17219: f64, t2812: f64, t8143: f64, t17169: f64, t17175: f64, t2586: f64, t953: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51164 = t2640 * t41521 * t4947;
    let t51169 = t2640 * t10894 * t16968;
    let t51189 = t17079 * t907;
    let t51322 = t2812 * t8143 * t17219;
    let t51325 = t2812 * t8143 * t17169;
    let t51349 = t953 * t2586 * t17175;
    (t51164, t51169, t51189, t51322, t51325, t51349)
}
