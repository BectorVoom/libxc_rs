//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta206 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk959;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk960;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta206(t5332: f64, t5465: f64, t1269: f64, t1287: f64, t1794: f64, t487: f64, t5284: f64, t3781: f64, t460: f64, t1248: f64, t3302: f64, t471: f64, t1811: f64, t473: f64, t1214: f64, t489: f64, t5412: f64, t1204: f64, t1234: f64, t1281: f64, t1285: f64, t1288: f64, t1291: f64, t1770: f64, t1818: f64, t1822: f64, t1825: f64, t3666: f64, t3670: f64, t3746: f64, t3755: f64, t490: f64, t5216: f64, t5326: f64, t5436: f64, t5443: f64, t5446: f64, t5449: f64, t5452: f64, t5459: f64, t5463: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5466, t5470, t5474, t5477, t5478, t5480) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk959(t5332, t5465, t1269, t1287, t1794, t487, t5284, t3781, t460, t1248, t3302, t471);
        let (t5481, t5486, t5487, t5491, t5494, t5497) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk960(t5332, t5480, t1811, t473, t1214, t1248, t1287, t489, t5412, t1204, t1234, t1281, t1285, t1288, t1291, t1770, t1818, t1822, t1825, t3666, t3670, t3746, t3755, t460, t490, t5216, t5326, t5436, t5443, t5446, t5449, t5452, t5459, t5463, t5466, t5470, t5474, t5478);
    (t5466, t5470, t5474, t5477, t5478, t5480, t5481, t5486, t5487, t5491, t5494, t5497)
}
