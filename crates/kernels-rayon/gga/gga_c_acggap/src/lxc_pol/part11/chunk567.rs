//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 567/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk567(t4010: f64, t4025: f64, t59: f64, t85: f64, t1357: f64, t807: f64, t2635: f64, t2644: f64, t2835: f64, t1390: f64, t224: f64, t2841: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4027 = (t4010 + t4025) * t59;
    let t4028 = t4027 * t85;
    let t4029 = 0.19751673498613801407e-1_f64 * t4028;
    let t4030 = t1357 * t807;
    let t4031 = 0.24415263074675393405e-3_f64 * t4030;
    let t4032 = 24.0_f64 * t2635;
    let t4036 = 2.0_f64 * t2644;
    let t4038 = 0.23392894490538584828e1_f64 * t2835;
    let t4039 = t224 * t1390;
    let t4040 = 8.0_f64 * t4039;
    let t4041 = 16.0_f64 * t2841;
    (t4027, t4029, t4031, t4032, t4036, t4038, t4040, t4041)
}
