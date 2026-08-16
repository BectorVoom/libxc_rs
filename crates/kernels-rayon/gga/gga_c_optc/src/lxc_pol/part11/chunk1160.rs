//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1160/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1160(t16988: f64, t896: f64, t7380: f64, t51399: f64, t935: f64, t19: f64, t17215: f64, t3907: f64, t42136: f64, t10838: f64, t17134: f64, t2721: f64) -> (f64, f64, f64, f64, f64) {
    let t52015 = t896 * t16988;
    let t52016 = t52015 * t7380;
    let t52037 = t51399 * t935;
    let t52061 = t52015 * t19;
    let t52111 = t3907 * t42136 * t17215;
    let t52138 = t2721 * t10838 * t17134;
    (t52016, t52037, t52061, t52111, t52138)
}
