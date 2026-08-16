//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1210/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1210(t315: f64, t40619: f64, t2134: f64, t1839: f64, t309: f64, t7932: f64, t7963: f64, t157: f64, t1937: f64, t406: f64, t2132: f64, t2138: f64, t322: f64, t9767: f64) -> (f64, f64, f64, f64) {
    let t40697 = t315 * t40619;
    let t40698 = t40697 * t2134;
    let t40703 = t1839 * t309;
    let t40705 = t7963 * t7932 * t40703;
    let t40709 = t1937 * t406 * t157;
    let t40721 = t2138 * t2132 * t9767 * t322;
    (t40698, t40705, t40709, t40721)
}
