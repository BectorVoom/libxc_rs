//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1085/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1085(t11661: f64, t276: f64, t40: f64, t5474: f64, t11679: f64, t11681: f64, t14880: f64, t14883: f64, t14885: f64, t14890: f64, t11665: f64, t11668: f64, t11672: f64, t3984: f64, t6614: f64, t694: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19425 = 0.32530743900905219526e-1_f64 * t11661;
    let t19430 = t40 * t5474 * t276;
    let t19431 = 2.0_f64 * t19430;
    let t19432 = 48.0_f64 * t11679;
    let t19433 = 96.0_f64 * t11681;
    let t19434 = 0.96319466275353142155e0_f64 * t14880;
    let t19435 = 0.43374325201206959368e-1_f64 * t14883;
    let t19436 = 0.32530743900905219526e-1_f64 * t14885;
    let t19437 = 0.43374325201206959368e-1_f64 * t14890;
    let t19438 = -6.0_f64 * t3984 * t6614 * t694 + t11665 + t11668 - t11672 + t19425 + t19431 + t19432 + t19433 + t19434 + t19435 + t19436 - t19437;
    (t19425, t19431, t19432, t19433, t19434, t19435, t19436, t19437, t19438)
}
