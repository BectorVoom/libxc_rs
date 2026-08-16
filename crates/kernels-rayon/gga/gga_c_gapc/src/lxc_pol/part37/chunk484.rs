//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 484/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk484(t314: f64, t5: f64, t4: f64, t291: f64, t297: f64, t512: f64, t641: f64, t916: f64, t1044: f64, t6: f64, t442: f64, t329: f64) -> (f64, f64, f64, f64) {
    let t2693 = t314 * t5;
    let t2694 = t2693 * t4;
    let t2695 = t512 * t291 * t297 * t2694;
    let t2698 = t916 * t641;
    let t2699 = t1044 * t291;
    let t2701 = t314 * t6;
    let t2702 = t2701 * t442;
    let t2703 = t2699 * t329 * t2702;
    (t2695, t2698, t2701, t2703)
}
