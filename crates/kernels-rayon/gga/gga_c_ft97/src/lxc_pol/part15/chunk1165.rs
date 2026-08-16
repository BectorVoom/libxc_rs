//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1165/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1165(t1212: f64, t21369: f64, t2665: f64, t446: f64, t835: f64, t88149: f64, t1234: f64, t83615: f64, t91: f64, t1091: f64, t21978: f64, t43381: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t89770 = t21369 * t1212;
    let t89772 = t446 * t2665 * t89770;
    let t89775 = t446 * t835 * t88149;
    let t89778 = t91 * t83615 * t1234;
    let t89779 = t1091 * t21978;
    let t89781 = t446 * t43381 * t89779;
    (t89770, t89772, t89775, t89778, t89779, t89781)
}
