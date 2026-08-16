//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 782/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk782(t4744: f64, t732: f64, t193: f64, t197: f64, t4599: f64, t745: f64, t1256: f64, t195: f64, t1924: f64, t4752: f64, t1320: f64, t3546: f64) -> (f64, f64, f64, f64, f64) {
    let t13526 = t732 * t4744;
    let t13536 = t193 * t745 * t4599 * t197;
    let t13538 = t195 * t1256;
    let t13543 = t193 * t1924 * t4752;
    let t13573 = t3546 * t1320;
    (t13526, t13536, t13538, t13543, t13573)
}
