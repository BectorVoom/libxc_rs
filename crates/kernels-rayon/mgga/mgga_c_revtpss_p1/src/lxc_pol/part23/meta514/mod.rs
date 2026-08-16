//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta514 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2017;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2018;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2019;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta514(t17235: f64, t19661: f64, t1042: f64, t1235: f64, t1238: f64, t1252: f64, t1261: f64, t17505: f64, t17569: f64, t21063: f64, t21085: f64, t21088: f64, t21091: f64, t21095: f64, t21102: f64, t21107: f64, t3667: f64, t5279: f64, t5320: f64, t5327: f64, t5384: f64, t6647: f64, t1248: f64, t3604: f64, t6688: f64, t3720: f64, t20266: f64, t5312: f64, t17475: f64, t20293: f64, t20318: f64, t5308: f64, t20310: f64, t20306: f64, t1260: f64, t6601: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21110, t21111, t21114) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2017(t17235, t19661, t1042, t1235, t1238, t1252, t1261, t17505, t17569, t21063, t21085, t21088, t21091, t21095, t21102, t21107, t3667, t5279, t5320, t5327, t5384, t6647);
        let t21119 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2018(t1248, t3604);
        let (t21120, t21121, t21126, t21129, t21134, t21137, t21140, t21143) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2019(t21119, t6688, t3720, t20266, t5312, t17475, t20293, t20318, t5308, t20310, t20306, t1260, t6601);
    (t21110, t21111, t21114, t21119, t21120, t21121, t21126, t21129, t21134, t21137, t21140, t21143)
}
