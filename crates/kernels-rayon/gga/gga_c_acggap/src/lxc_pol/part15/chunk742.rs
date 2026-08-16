//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 742/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk742(t2230: f64, t7990: f64, t2226: f64, t7987: f64, t2147: f64, t2229: f64, t463: f64, t2138: f64, t2132: f64, t2225: f64, t322: f64, t7896: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8076 = t7990 * t2230;
    let t8078 = t7987 * t2226;
    let t8081 = t2147 * t2229 * t463;
    let t8082 = t2138 * t8081;
    let t8085 = t2132 * t2225 * t322;
    let t8087 = 0.34694512752820797848e1_f64 * t7896 * t8085;
    (t8076, t8078, t8081, t8082, t8085, t8087)
}
