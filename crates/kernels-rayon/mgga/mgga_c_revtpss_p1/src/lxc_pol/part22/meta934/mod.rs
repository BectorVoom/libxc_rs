//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta934 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3165;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3166;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta934(t17769: f64, t3647: f64, t1235: f64, t371: f64, t5318: f64, t676: f64, t225: f64, t56331: f64, t1789: f64, t2434: f64, t1261: f64, t16746: f64, t247: f64, t3634: f64, t1012: f64, t44958: f64, t13026: f64, t140: f64, t1222: f64, t16715: f64, t1224: f64, t5052: f64, t697: f64, t12915: f64, t17344: f64, t17345: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57451, t57463, t57465, t57471, t57478) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3165(t17769, t3647, t1235, t371, t5318, t676, t225, t56331, t1789, t2434, t1261, t16746, t247, t3634);
        let (t57480, t57484, t57486, t57490, t57508) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3166(t1012, t44958, t13026, t140, t1222, t16715, t1224, t5052, t697, t12915, t17344, t17345, t247);
    (t57451, t57463, t57465, t57471, t57478, t57480, t57484, t57486, t57490, t57508)
}
