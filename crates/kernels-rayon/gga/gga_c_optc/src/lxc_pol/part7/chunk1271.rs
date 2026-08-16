//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1271/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1271(t2985: f64, t2992: f64, t2995: f64, t1032: f64, t8581: f64, t8583: f64, t26153: f64, t3020: f64, t8686: f64, t1036: f64, t8896: f64, t1057: f64) -> (f64, f64, f64, f64) {
    let t26201 = t2985 * t2992;
    let t26203 = 12.0_f64 * t26201 * t2995;
    let t26204 = t1032 * t8581;
    let t26206 = 0.38596378373162651572e3_f64 * t26204 * t8583;
    let t26209 = 0.57894567559743977359e3_f64 * t8686 * t26153 * t3020;
    let t26210 = t8896 * t1036;
    let t26212 = 4.0_f64 * t26210 * t1057;
    (t26203, t26206, t26209, t26212)
}
