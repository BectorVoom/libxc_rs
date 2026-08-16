//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 718/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk718(t6807: f64, t6808: f64, t6812: f64, t6841: f64, t138: f64, t2053: f64, t637: f64, t658: f64, t120: f64, t2086: f64, t1928: f64, t616: f64) -> (f64, f64, f64, f64, f64) {
    let t6843 = t6807 + t6808 + t6812 + t6841;
    let t6847 = t2053 * t138;
    let t6850 = t637 * t658;
    let t6855 = t120 * t2086;
    let t6856 = t1928 * t616;
    (t6843, t6847, t6850, t6855, t6856)
}
