//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1804;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1805;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1806;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1807;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1808;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta339(t11200: f64, t378: f64, t3043: f64, t3042: f64, t993: f64, t1071: f64, t989: f64, t3056: f64, t988: f64, t1031: f64, t3145: f64, t334: f64, t368: f64, t3153: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11201, t11210, t11213) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1804(t11200, t378, t3043, t3042, t993);
        let (t11214, t11220, t11223) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1805(t11213, t378, t1071, t989, t3056, t988);
        let t11224 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1806(t11223, t378);
        let (t11238, t11239) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1807(t1031);
        let (t11243, t11249) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1808(t3145, t334, t368, t3153, t73);
    (t11201, t11210, t11213, t11214, t11220, t11223, t11224, t11238, t11239, t11243, t11249)
}
