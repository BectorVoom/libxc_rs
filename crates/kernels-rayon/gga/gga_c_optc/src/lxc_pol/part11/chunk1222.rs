//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1222/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1222(t13248: f64, t4599: f64, t6931: f64, t13214: f64, t4595: f64, t2034: f64, t1256: f64, t48528: f64, t13209: f64, t4649: f64, t162: f64, t13174: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t56123 = t13248 * t4599;
    let t56124 = t6931 * t56123;
    let t56127 = t13214 * t4595;
    let t56128 = t2034 * t56127;
    let t56131 = t48528 * t1256;
    let t56132 = t2034 * t56131;
    let t56135 = t13209 * t4649;
    let t56136 = t162 * t56135;
    let t56139 = t13174 * t4649;
    (t56123, t56124, t56127, t56128, t56131, t56132, t56135, t56136, t56139)
}
