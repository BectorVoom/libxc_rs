//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 976/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk976(t1264: f64, t2131: f64, t2133: f64, t2147: f64, t3645: f64, t611: f64, t7908: f64, t7990: f64, t694: f64, t7278: f64, t839: f64, t10409: f64, t1679: f64, t467: f64) -> (f64, f64, f64, f64, f64) {
    let t32219 = t2131 * t2147 * t2133 * t1264;
    let t32222 = 0.65854491829355115987e0_f64 * t3645 * t611;
    let t32223 = t7990 * t7908;
    let t32246 = t694 * t7278 * t839;
    let t32249 = t1679 * t10409 * t467;
    (t32219, t32222, t32223, t32246, t32249)
}
