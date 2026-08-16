//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2161;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2162;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2163;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2164;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2165;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta470(t12131: f64, t3095: f64, t15691: f64, t372: f64, t4823: f64, t3096: f64, t1087: f64, t11773: f64, t4801: f64, t4181: f64, t4786: f64, t1062: f64, t4857: f64, t11986: f64, t1592: f64, t247: f64, t1063: f64, t11940: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15692, t15693, t15696) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2161(t12131, t3095, t15691, t372, t4823);
        let (t15697, t15700) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2162(t15696, t3096, t1087, t11773);
        let (t15701, t15702) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2163(t372, t4801, t4181, t4786);
        let (t15703, t15707) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2164(t15701, t15702, t1062, t4857);
        let (t15711, t15712, t15716) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2165(t11986, t1592, t247, t1063, t1062, t11940);
    (t15692, t15693, t15696, t15697, t15700, t15701, t15702, t15703, t15707, t15711, t15712, t15716)
}
