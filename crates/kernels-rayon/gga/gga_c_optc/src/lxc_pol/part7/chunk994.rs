//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 994/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk994(t21931: f64, t1948: f64, t616: f64, t1880: f64, t1953: f64, t201: f64, t21907: f64, t21911: f64, t21913: f64, t21915: f64, t21920: f64, t21929: f64, t3316: f64, t3318: f64, t3539: f64, t6672: f64, t7159: f64, t755: f64, t9361: f64, t95: f64, t9548: f64) -> (f64, f64, f64) {
    let t21932 = 960.0_f64 * t21931;
    let t21933 = t1948 * t616;
    let t21937 = -t21907 + 0.93041573165652349788e-1_f64 * t95 * t9361 * t1948 + 6.0_f64 * t21911 + 6.0_f64 * t21913 + 3.0_f64 * t3316 * t3318 * t21915 * t1953 - 14.0_f64 * t21920 + 2.0_f64 * t3316 * t3318 * t6672 * t755 * t201 + 6.0_f64 * t9548 * t7159 + 6.0_f64 * t21929 + t21932 + 0.18608314633130469958e0_f64 * t3539 * t1880 * t21933;
    (t21932, t21933, t21937)
}
