//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta206 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk959;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk960;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta206<F: Float>(t5332: F, t5465: F, t1269: F, t1287: F, t1794: F, t487: F, t5284: F, t3781: F, t460: F, t1248: F, t3302: F, t471: F, t1811: F, t473: F, t1214: F, t489: F, t5412: F, t1204: F, t1234: F, t1281: F, t1285: F, t1288: F, t1291: F, t1770: F, t1818: F, t1822: F, t1825: F, t3666: F, t3670: F, t3746: F, t3755: F, t490: F, t5216: F, t5326: F, t5436: F, t5443: F, t5446: F, t5449: F, t5452: F, t5459: F, t5463: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5466, t5470, t5474, t5477, t5478, t5480) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk959::<F>(t5332, t5465, t1269, t1287, t1794, t487, t5284, t3781, t460, t1248, t3302, t471);
        let (t5481, t5486, t5487, t5491, t5494, t5497) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk960::<F>(t5332, t5480, t1811, t473, t1214, t1248, t1287, t489, t5412, t1204, t1234, t1281, t1285, t1288, t1291, t1770, t1818, t1822, t1825, t3666, t3670, t3746, t3755, t460, t490, t5216, t5326, t5436, t5443, t5446, t5449, t5452, t5459, t5463, t5466, t5470, t5474, t5478);
    (t5466, t5470, t5474, t5477, t5478, t5480, t5481, t5486, t5487, t5491, t5494, t5497)
}
