//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta268 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1200;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1201;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1202;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta268(t1444: f64, t2022: f64, t7296: f64, t1385: f64, t1426: f64, t1398: f64, t543: f64, t545: f64, t7274: f64, t2028: f64, t1445: f64, t2027: f64, t2030: f64, t213: f64, t561: f64, t7245: f64, t7248: f64, t7275: f64, t7279: f64, t7288: f64, t7291: f64, t7292: f64, t7295: f64, t532: f64, t1450: f64, t2014: f64, t1448: f64, t4147: f64, t2034: f64, t118: f64, t1310: f64, t1453: f64, t1932: f64, t2007: f64, t2011: f64, t508: f64, t569: f64, t649: f64, t651: f64, t671: f64, t6983: f64, t6985: f64, t6990: f64, t6992: f64, t6995: f64, t7005: f64, t7007: f64, t7221: f64, t7231: f64, t7236: f64, t7241: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7298, t7301) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1200(t1444, t2022, t7296, t1385, t1426);
        let (t7303, t7304, t7307, t7308, t7311) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1201(t1398, t2022, t543, t7301, t545, t7274, t2028, t1445, t2027, t2030, t213, t561, t7245, t7248, t7275, t7279, t7288, t7291, t7292, t7295, t7298);
        let (t7312, t7313, t7315, t7316, t7318) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1202(t532, t7311, t1450, t2014, t1448, t4147, t2034, t118, t1310, t1453, t1932, t2007, t2011, t508, t569, t649, t651, t671, t6983, t6985, t6990, t6992, t6995, t7005, t7007, t7221, t7231, t7236, t7241);
    (t7298, t7301, t7303, t7304, t7307, t7308, t7311, t7312, t7313, t7315, t7316, t7318)
}
