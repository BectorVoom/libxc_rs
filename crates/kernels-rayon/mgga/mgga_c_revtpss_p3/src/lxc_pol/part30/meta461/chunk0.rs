//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1757/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1757(t1927: f64, t644: f64, t4144: f64, t9593: f64, t2034: f64, t2014: f64, t10416: f64, t1937: f64, t13435: f64, t2322: f64, t6993: f64, t196: f64, t197: f64, t3821: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25163 = t1927 * t644;
    let t25177 = t9593 * t4144;
    let t25178 = t2034 * t25177;
    let t25180 = 2.0_f64 * t2014 * t25178;
    let t25182 = 2.0_f64 * t10416 * t1937;
    let t25184 = 4.0_f64 * t13435 * t1937;
    let t25186 = 4.0_f64 * t2322 * t6993;
    let t25188 = t3821 * t196 * t197;
    (t25163, t25177, t25178, t25180, t25182, t25184, t25186, t25188)
}
