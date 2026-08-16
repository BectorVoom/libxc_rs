//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1538/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1538(t15098: f64, t2924: f64, t1596: f64, t2873: f64, t2876: f64, t1614: f64, t2942: f64, t11354: f64, t1600: f64, t2881: f64, t11358: f64, t2880: f64, t4606: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15100 = 6.0_f64 * t2924 * t15098;
    let t15101 = t1596 * t2873;
    let t15103 = 2.0_f64 * t15101 * t2876;
    let t15104 = t1614 * t2942;
    let t15107 = t11354 * t1600;
    let t15108 = t15107 * t2881;
    let t15110 = t11358 * t1600;
    let t15111 = t15110 * t2881;
    let t15113 = t2880 * t4606;
    (t15100, t15103, t15104, t15108, t15111, t15113)
}
