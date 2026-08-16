//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta670 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2637;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2638;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta670(t17633: f64, t6638: f64, t3626: f64, t12884: f64, t247: f64, t6421: f64, t1261: f64, t20302: f64, t5312: f64, t20298: f64, t1785: f64, t5390: f64, t20703: f64, t3719: f64, t5357: f64, t5373: f64, t140: f64, t6658: f64, t1222: f64, t6662: f64, t1774: f64, t5284: f64, t1250: f64, t3720: f64, t1266: f64, t17629: f64, t3625: f64, t3718: f64, t5381: f64, t5384: f64, t5397: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21227, t21228, t21233, t21234, t21236, t21239, t21242) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2637(t17633, t6638, t3626, t12884, t247, t6421, t1261, t20302, t5312, t20298, t1785, t5390);
        let (t21246, t21249, t21252, t21255, t21257) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2638(t20703, t247, t3719, t5357, t5373, t140, t6658, t1222, t6662, t1774, t5284);
        let (t21258, t21259, t21264) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2639(t1250, t21257, t3720, t1222, t1266, t17629, t21228, t21234, t21236, t21239, t21242, t21246, t21249, t21252, t21255, t3625, t3718, t5381, t5384, t5397);
    (t21227, t21228, t21233, t21242, t21246, t21257, t21258, t21259, t21264)
}
