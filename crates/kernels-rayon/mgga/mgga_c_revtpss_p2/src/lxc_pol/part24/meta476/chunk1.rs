//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1461/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1461(t3201: f64, t6318: f64, t1011: f64, t6292: f64, t697: f64, t19649: f64, t372: f64, t6284: f64, t6288: f64, t3091: f64, t43240: f64, t6267: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t66141 = t6318 * t3201;
    let t66218 = t1011 * t697 * t6292;
    let t66306 = t372 * t19649;
    let t66547 = t1011 * t697 * t6284;
    let t66721 = t1011 * t697 * t6288;
    let t66763 = t3091 * t43240 * t6267;
    (t66141, t66218, t66306, t66547, t66721, t66763)
}
