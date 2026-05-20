//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta678 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2658;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2659;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta678<F: Float>(t1770: F, t5477: F, t1248: F, t17847: F, t20956: F, t17854: F, t1280: F, t20721: F, t5284: F, t5464: F, t5332: F, t1287: F, t20856: F, t1794: F, t5412: F, t5245: F, t5486: F, t1204: F, t1234: F, t12717: F, t1281: F, t1285: F, t17192: F, t17289: F, t17846: F, t17853: F, t1818: F, t20850: F, t3666: F, t3670: F, t3746: F, t5326: F, t5436: F, t5449: F, t5452: F, t5459: F, t5463: F, t5474: F, t5481: F, t6723: F, t6735: F, t6741: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t21579, t21583, t21587, t21592, t21596, t21599) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2658::<F>(t1770, t5477, t1248, t17847, t20956, t17854, t1280, t20721, t5284, t5464, t5332, t1287, t20856);
        let (t21607, t21610, t21615) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2659::<F>(t1287, t1794, t5412, t5245, t5486, t1204, t1234, t12717, t1281, t1285, t17192, t17289, t17846, t17853, t1818, t20850, t21579, t21583, t21587, t21592, t21596, t21599, t3666, t3670, t3746, t5326, t5436, t5449, t5452, t5459, t5463, t5474, t5481, t6723, t6735, t6741);
    (t21579, t21583, t21587, t21592, t21596, t21599, t21607, t21610, t21615)
}
