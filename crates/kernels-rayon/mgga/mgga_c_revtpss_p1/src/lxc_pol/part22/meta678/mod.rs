//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta678 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2658;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2659;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta678(t1770: f64, t5477: f64, t1248: f64, t17847: f64, t20956: f64, t17854: f64, t1280: f64, t20721: f64, t5284: f64, t5464: f64, t5332: f64, t1287: f64, t20856: f64, t1794: f64, t5412: f64, t5245: f64, t5486: f64, t1204: f64, t1234: f64, t12717: f64, t1281: f64, t1285: f64, t17192: f64, t17289: f64, t17846: f64, t17853: f64, t1818: f64, t20850: f64, t3666: f64, t3670: f64, t3746: f64, t5326: f64, t5436: f64, t5449: f64, t5452: f64, t5459: f64, t5463: f64, t5474: f64, t5481: f64, t6723: f64, t6735: f64, t6741: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21579, t21583, t21587, t21592, t21596, t21599) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2658(t1770, t5477, t1248, t17847, t20956, t17854, t1280, t20721, t5284, t5464, t5332, t1287, t20856);
        let (t21607, t21610, t21615) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2659(t1287, t1794, t5412, t5245, t5486, t1204, t1234, t12717, t1281, t1285, t17192, t17289, t17846, t17853, t1818, t20850, t21579, t21583, t21587, t21592, t21596, t21599, t3666, t3670, t3746, t5326, t5436, t5449, t5452, t5459, t5463, t5474, t5481, t6723, t6735, t6741);
    (t21579, t21583, t21587, t21592, t21596, t21599, t21607, t21610, t21615)
}
