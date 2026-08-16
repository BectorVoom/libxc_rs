//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2459/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2459(t138: f64, t1444: f64, t9302: f64, t9674: f64, t10162: f64, t9303: f64, t3903: f64, t9292: f64, t3906: f64, t3907: f64, t39494: f64, t10115: f64, t1421: f64) -> (f64, f64, f64, f64, f64) {
    let t47487 = t9674 * t138 * t9302 * t1444;
    let t47495 = t9303 * t10162;
    let t47497 = t9292 * t3903;
    let t47504 = 0.20561456923286030469e-1_f64 * t3906 * t3907 * t39494;
    let t47512 = t10115 * t1421;
    (t47487, t47495, t47497, t47504, t47512)
}
