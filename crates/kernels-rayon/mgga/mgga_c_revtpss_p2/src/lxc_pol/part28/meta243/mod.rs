//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta243 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1094;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1095;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1096;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1097;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta243(t460: f64, t5477: f64, t1248: f64, t3302: f64, t471: f64, t5332: f64, t1811: f64, t473: f64, t1214: f64, t1287: f64, t489: f64, t5412: f64, t1204: f64, t1234: f64, t1281: f64, t1285: f64, t1288: f64, t1291: f64, t1770: f64, t1818: f64, t1822: f64, t1825: f64, t3666: f64, t3670: f64, t3746: f64, t3755: f64, t490: f64, t5216: f64, t5326: f64, t5436: f64, t5443: f64, t5446: f64, t5449: f64, t5452: f64, t5459: f64, t5463: f64, t5466: f64, t5470: f64, t5474: f64, t1277: f64, t1210: f64, t1215: f64, t1271: f64, t1274: f64, t1295: f64, t1775: f64, t1813: f64, t1829: f64, t3556: f64, t3561: f64, t3567: f64, t3572: f64, t3732: f64, t495: f64, t5220: f64, t5225: f64, t5231: f64, t5237: f64, t5246: f64, t5251: f64, t5414: f64, t5417: f64, t5423: f64, t5429: f64, t1832: f64, t3801: f64, t1298: f64, t1300: f64, t198: f64, t336: f64, t5023: f64, t5062: f64, t5065: f64, t5067: f64, t5070: f64, t5107: f64, t5111: f64, t5189: f64, t5191: f64, t5194: f64, t5196: f64, t5200: f64, t5204: f64, t5209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5478, t5480, t5481, t5486, t5487, t5491, t5494) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1094(t460, t5477, t1248, t3302, t471, t5332, t1811, t473, t1214, t1287, t489, t5412);
        let t5497 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1095(t1204, t1234, t1281, t1285, t1288, t1291, t1770, t1818, t1822, t1825, t3666, t3670, t3746, t3755, t460, t490, t5216, t5326, t5436, t5443, t5446, t5449, t5452, t5459, t5463, t5466, t5470, t5474, t5478, t5481, t5487, t5491, t5494);
        let (t5498, t5501) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1096(t1277, t5497, t1204, t1210, t1215, t1271, t1274, t1295, t1770, t1775, t1813, t1829, t3556, t3561, t3567, t3572, t3732, t460, t495, t5216, t5220, t5225, t5231, t5237, t5246, t5251, t5414, t5417, t5423, t5429);
        let (t5505, t5508) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1097(t1832, t3801, t1298, t1300, t198, t336, t5023, t5062, t5065, t5067, t5070, t5107, t5111, t5189, t5191, t5194, t5196, t5200, t5204, t5209, t5501);
    (t5478, t5480, t5481, t5486, t5487, t5491, t5494, t5497, t5498, t5501, t5505, t5508)
}
