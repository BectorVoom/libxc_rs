//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta394 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1970;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1971;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1972;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1973;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta394(t4004: f64, t5673: f64, t5674: f64, t9840: f64, t1868: f64, t3829: f64, t828: f64, t9942: f64, t5608: f64, t5675: f64, t9934: f64, t2661: f64, t3936: f64, t5704: f64, t3924: f64, t2482: f64, t4000: f64, t814: f64, t136: f64, t550: f64, t220: f64, t124: f64, t1882: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13817, t13821, t13824, t13826, t13830, t13832) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1970(t4004, t5673, t5674, t9840, t1868, t3829, t828, t9942, t5608, t5675, t9934, t2661);
        let (t13834, t13841, t13845) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1971(t3936, t4004, t5704, t3924, t2482, t4000, t814);
        let t13847 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1972(t136, t550, t220);
        let t13848 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1973(t124, t1882);
    (t13817, t13821, t13824, t13826, t13830, t13832, t13834, t13841, t13845, t13847, t13848)
}
