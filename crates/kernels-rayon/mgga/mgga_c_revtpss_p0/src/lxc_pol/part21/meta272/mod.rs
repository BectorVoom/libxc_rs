//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta272 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1491;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1492;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta272(t3940: f64, t9962: f64, t1371: f64, t3889: f64, t800: f64, t221: f64, t3924: f64, t4019: f64, t4018: f64, t3930: f64, t4059: f64, t1386: f64, t2482: f64, t596: f64, t4021: f64, t1398: f64, t1412: f64, t3938: f64, t3992: f64, t2661: f64, t1353: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9963, t9966, t9970, t9971, t9973, t9976) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1491(t3940, t9962, t1371, t3889, t800, t221, t3924, t4019, t4018, t3930, t4059, t1386, t2482, t596);
        let (t9977, t9979, t9981, t9982, t9984) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1492(t4021, t9976, t1398, t1412, t3938, t3992, t2661, t1353, t3889);
    (t9963, t9966, t9970, t9971, t9973, t9976, t9977, t9979, t9981, t9982, t9984)
}
