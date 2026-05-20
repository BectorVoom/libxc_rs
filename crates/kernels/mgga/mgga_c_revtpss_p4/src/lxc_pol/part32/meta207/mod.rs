//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta207 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk899;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk900;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk901;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk902;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta207<F: Float>(t460: F, t5477: F, t1248: F, t3302: F, t471: F, t5332: F, t1811: F, t473: F, t1214: F, t1287: F, t489: F, t5412: F, t1204: F, t1234: F, t1281: F, t1285: F, t1288: F, t1291: F, t1770: F, t1818: F, t1822: F, t1825: F, t3666: F, t3670: F, t3746: F, t3755: F, t490: F, t5216: F, t5326: F, t5436: F, t5443: F, t5446: F, t5449: F, t5452: F, t5459: F, t5463: F, t5466: F, t5470: F, t5474: F, t1277: F, t1210: F, t1215: F, t1271: F, t1274: F, t1295: F, t1775: F, t1813: F, t1829: F, t3556: F, t3561: F, t3567: F, t3572: F, t3732: F, t495: F, t5220: F, t5225: F, t5231: F, t5237: F, t5246: F, t5251: F, t5414: F, t5417: F, t5423: F, t5429: F, t1832: F, t3801: F, t1298: F, t1300: F, t198: F, t336: F, t5023: F, t5062: F, t5065: F, t5067: F, t5070: F, t5107: F, t5111: F, t5189: F, t5191: F, t5194: F, t5196: F, t5200: F, t5204: F, t5209: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5478, t5480, t5481, t5486, t5487, t5491, t5494) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk899::<F>(t460, t5477, t1248, t3302, t471, t5332, t1811, t473, t1214, t1287, t489, t5412);
        let t5497 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk900::<F>(t1204, t1234, t1281, t1285, t1288, t1291, t1770, t1818, t1822, t1825, t3666, t3670, t3746, t3755, t460, t490, t5216, t5326, t5436, t5443, t5446, t5449, t5452, t5459, t5463, t5466, t5470, t5474, t5478, t5481, t5487, t5491, t5494);
        let (t5498, t5501) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk901::<F>(t1277, t5497, t1204, t1210, t1215, t1271, t1274, t1295, t1770, t1775, t1813, t1829, t3556, t3561, t3567, t3572, t3732, t460, t495, t5216, t5220, t5225, t5231, t5237, t5246, t5251, t5414, t5417, t5423, t5429);
        let (t5505, t5508) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk902::<F>(t1832, t3801, t1298, t1300, t198, t336, t5023, t5062, t5065, t5067, t5070, t5107, t5111, t5189, t5191, t5194, t5196, t5200, t5204, t5209, t5501);
    (t5478, t5480, t5481, t5486, t5487, t5491, t5494, t5497, t5498, t5501, t5505, t5508)
}
