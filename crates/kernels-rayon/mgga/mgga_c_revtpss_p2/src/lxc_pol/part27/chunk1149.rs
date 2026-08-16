//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1149/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1149(t25956: f64, t26087: f64, t532: f64, t1450: f64, t2014: f64, t2042: f64, t4158: f64, t1459: f64, t7331: f64, t7334: f64, t1936: f64, t2327: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26088 = t25956 + t26087;
    let t26089 = t532 * t26088;
    let t26090 = t26089 * t1450;
    let t26091 = t2014 * t26090;
    let t26115 = 3.0_f64 * t4158 * t2042;
    let t26117 = 12.0_f64 * t1459 * t7331;
    let t26119 = 6.0_f64 * t1459 * t7334;
    let t26120 = t2327 * t1936;
    (t26088, t26089, t26090, t26091, t26115, t26117, t26119, t26120)
}
