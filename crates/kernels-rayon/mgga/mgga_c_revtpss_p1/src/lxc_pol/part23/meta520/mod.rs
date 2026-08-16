//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2029;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2030;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2031;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta520(t20703: f64, t247: f64, t3719: f64, t5357: f64, t5373: f64, t140: f64, t6658: f64, t1222: f64, t6662: f64, t1774: f64, t5284: f64, t1250: f64, t3720: f64, t1266: f64, t17629: f64, t21228: f64, t21234: f64, t21236: f64, t21239: f64, t21242: f64, t3625: f64, t3718: f64, t5381: f64, t5384: f64, t5397: f64, t20747: f64, t369: f64, t6593: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21246, t21249, t21251, t21252, t21254, t21255, t21257, t21258) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2029(t20703, t247, t3719, t5357, t5373, t140, t6658, t1222, t6662, t1774, t5284, t1250);
        let (t21259, t21264) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2030(t21258, t3720, t1222, t1266, t17629, t21228, t21234, t21236, t21239, t21242, t21246, t21249, t21252, t21255, t3625, t3718, t5381, t5384, t5397);
        let (t21267, t21271) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2031(t20747, t247, t3719, t369, t6593, t475);
    (t21246, t21249, t21251, t21252, t21254, t21255, t21257, t21258, t21259, t21264, t21267, t21271)
}
