//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1932;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1933;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1934;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1935;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta378(t13396: f64, t70: f64, t4181: f64, t627: f64, t13312: f64, t36: f64, t4187: f64, t1470: f64, t2291: f64, t13389: f64, t13393: f64, t1494: f64, t2292: f64, t4182: f64, t4188: f64, t4191: f64, t4238: f64, t628: f64, t641: f64, t71: f64, t85: f64, t13363: f64, t10298: f64, t10301: f64, t10309: f64, t13267: f64, t13269: f64, t13272: f64, t13283: f64, t13286: f64, t13289: f64, t1497: f64, t2242: f64, t2247: f64, t2248: f64, t2315: f64, t4173: f64, t4178: f64, t4241: f64, t603: f64, t644: f64, t91: f64, t5: f64, t117: f64, t116: f64, t4245: f64, t1501: f64, t2327: f64, t648: f64, t670: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13397, t13400, t13405, t13406, t13409, t13414, t13419) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1932(t13396, t70, t4181, t627, t13312, t36, t4187, t1470, t2291, t13389, t13393, t1494, t2292, t4182, t4188, t4191, t4238, t628, t641, t71, t85);
        let (t13420, t13423) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1933(t13363, t13419, t10298, t10301, t10309, t13267, t13269, t13272, t13283, t13286, t13289, t1497, t2242, t2247, t2248, t2315, t4173, t4178, t4241, t603, t644, t91);
        let (t13424, t13425, t13426) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1934(t5, t13423, t117, t116, t4245);
        let (t13429, t13435) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1935(t1501, t2327, t648, t670);
    (t13397, t13400, t13405, t13406, t13409, t13414, t13420, t13424, t13425, t13426, t13429, t13435)
}
