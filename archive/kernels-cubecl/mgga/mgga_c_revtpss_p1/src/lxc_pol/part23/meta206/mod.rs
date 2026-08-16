//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta206 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1224;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1225;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1226;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1227;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1228;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta206<F: Float>(t2282: F, t5819: F, t5825: F, t60: F, t1480: F, t1483: F, t2290: F, t44: F, t56: F, t5835: F, t5838: F, t5843: F, t61: F, t38: F, t2299: F, t633: F, t2306: F, t637: F, t77: F, t1471: F, t1487: F, t1494: F, t5820: F, t5827: F, t5830: F, t71: F, t85: F, t5: F, t1497: F, t2247: F, t4173: F, t5812: F, t5816: F, t603: F, t91: F, t117: F, t1518: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t5848, t5851, t5854) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1224::<F>(t2282, t5819, t5825, t60, t1480, t1483, t2290, t44, t56, t5835, t5838, t5843, t61);
        let (t5855, t5869) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1225::<F>(t38, t5854, t2299, t5819, t5825, t633, t2306, t637, t77);
        let t5872 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1226::<F>(t1471, t1487, t1494, t5820, t5827, t5830, t5855, t5869, t71, t85);
        let (t5876, t5877) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1227::<F>(t5, t1497, t2247, t4173, t5812, t5816, t5872, t603, t91, t117);
        let t5883 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1228::<F>(t1518);
    (t5848, t5851, t5854, t5855, t5869, t5872, t5876, t5877, t5883)
}
