//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 854/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk854(t29991: f64, t621: f64, t2122: f64, t394: f64, t2130: f64, t7923: f64, t2149: f64, t7922: f64, t861: f64, t2140: f64, t3054: f64, t609: f64, t865: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29992 = t29991 * t621;
    let t29997 = t394 * t2122;
    let t30005 = t7923 * t2130;
    let t30006 = t30005 * t2149;
    let t30009 = t7922 * t861 * t2130;
    let t30011 = 0.52041769129231196772e1_f64 * t30009 * t2140;
    let t30015 = 0.39512695097613069591e1_f64 * t3054 * t609 * t865;
    (t29992, t29997, t30005, t30006, t30011, t30015)
}
