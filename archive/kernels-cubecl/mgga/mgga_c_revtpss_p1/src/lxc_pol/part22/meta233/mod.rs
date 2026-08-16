//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1465;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1466;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1467;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1468;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1469;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1470;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta233<F: Float>(t5674: F, t5675: F, t5673: F, t1388: F, t1410: F, t3931: F, t3956: F, t4022: F, t4064: F, t5606: F, t5611: F, t5614: F, t5619: F, t5623: F, t5625: F, t5629: F, t5661: F, t5666: F, t5671: F, t1873: F, t3957: F, t1353: F, t1872: F, t800: F, t124: F, t5591: F, t3938: F, t3936: F, t1399: F, t125: F, t1868: F, t1370: F, t3934: F, t3944: F, t3950: F, t3953: F, t3958: F, t3967: F, t3976: F, t3982: F, t3987: F, t3990: F, t3996: F, t225: F, t1892: F, t213: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5677, t5680) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1465::<F>(t5674, t5675, t5673, t1388, t1410, t3931, t3956, t4022, t4064, t5606, t5611, t5614, t5619, t5623, t5625, t5629, t5661, t5666, t5671);
        let (t5681, t5686, t5689, t5690, t5697, t5701) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1466::<F>(t1873, t3957, t1353, t1872, t800, t124, t5591, t3938, t5674, t3936, t1399, t5673);
        let t5704 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1467::<F>(t125, t1868);
        let (t5706, t5709) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1468::<F>(t1399, t5704, t3936, t1370, t3934, t3944, t3950, t3953, t3958, t3967, t3976, t3982, t3987, t3990, t3996, t5681, t5686, t5690, t5697, t5701);
        let t5710 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1469::<F>(t5680, t5709);
        let (t5711, t5715) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1470::<F>(t225, t5710, t1892, t213);
    (t5677, t5681, t5686, t5689, t5690, t5697, t5701, t5704, t5706, t5710, t5711, t5715)
}
