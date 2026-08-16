//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1244/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1244(t33551: f64, t7963: f64, t8306: f64, t33787: f64, t2131: f64, t2147: f64, t2394: f64, t847: f64, t33574: f64, t8085: f64, t7987: f64, t9159: f64) -> (f64, f64, f64, f64, f64) {
    let t38441 = 0.17347256376410398924e1_f64 * t7963 * t8306 * t33551;
    let t38443 = t7963 * t8306 * t33787;
    let t38453 = t2131 * t2147 * t2394 * t847;
    let t38455 = t33574 * t8085;
    let t38458 = 0.34694512752820797848e1_f64 * t7987 * t9159;
    (t38441, t38443, t38453, t38455, t38458)
}
