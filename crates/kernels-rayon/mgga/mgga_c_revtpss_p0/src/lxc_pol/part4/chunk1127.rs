//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1127/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1127(t13881: f64, t13882: f64, t13884: f64, t13889: f64, t225: f64, t1392: f64, t73: f64, t13768: f64, t3829: f64, t1412: f64, t5591: f64, t1353: f64) -> (f64, f64, f64, f64) {
    let t13892 = (t13881 + t13882 + t13884 + t13889) * t225;
    let t13902 = t1392 * t73;
    let t13907 = t13768 * t3829;
    let t13910 = t1412 * t5591;
    let t13911 = t13910 * t1353;
    (t13892, t13902, t13907, t13911)
}
