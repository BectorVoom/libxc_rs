//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2968/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2968(t1889: f64, t46595: f64, t1353: f64, t13767: f64, t2661: f64, t48432: f64, t13768: f64, t3889: f64, t13977: f64, t9962: f64, t13847: f64, t1399: f64, t48919: f64, t9816: f64) -> (f64, f64, f64, f64, f64) {
    let t48947 = t46595 * t1889;
    let t48951 = t2661 * t13767 * t48432 * t1353;
    let t48955 = t2661 * t13767 * t13768 * t3889;
    let t48971 = t9962 * t13977;
    let t48975 = t9816 * t13847 * t48919 * t1399;
    (t48947, t48951, t48955, t48971, t48975)
}
