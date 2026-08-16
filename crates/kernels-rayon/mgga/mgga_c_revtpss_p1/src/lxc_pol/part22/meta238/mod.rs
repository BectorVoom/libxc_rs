//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta238 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1492;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1493;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta238(t38: f64, t5854: f64, t2299: f64, t5819: f64, t5825: f64, t633: f64, t2306: f64, t637: f64, t77: f64, t1471: f64, t1487: f64, t1494: f64, t5820: f64, t5827: f64, t5830: f64, t71: f64, t85: f64, t5: f64, t1497: f64, t2247: f64, t4173: f64, t5812: f64, t5816: f64, t603: f64, t91: f64, t117: f64, t1518: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5855, t5860, t5862, t5864, t5866, t5869, t5872) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1492(t38, t5854, t2299, t5819, t5825, t633, t2306, t637, t77, t1471, t1487, t1494, t5820, t5827, t5830, t71, t85);
        let (t5876, t5877, t5883) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1493(t5, t1497, t2247, t4173, t5812, t5816, t5872, t603, t91, t117, t1518);
    (t5855, t5860, t5862, t5864, t5866, t5869, t5872, t5876, t5877, t5883)
}
