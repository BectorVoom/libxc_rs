//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 771/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk771(t3: f64, t5: f64, t8785: f64, t8784: f64, t8789: f64, t3100: f64, t664: f64, t3044: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t9059 = t3 * t5;
    let t9060 = t9059 * t8785;
    let t9061 = t8784 * t9060;
    let t9062 = t9061 * t8789;
    let t9064 = t3100 * t664;
    let t9066 = t3044 * pi;
    (t9059, t9060, t9061, t9062, t9064, t9066)
}
