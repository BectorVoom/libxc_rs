//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta245 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1090;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1091;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1092;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1093;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1094;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1095;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1096;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta245<F: Float>(t5332: F, t5480: F, t1811: F, t473: F, t1214: F, t1248: F, t1287: F, t489: F, t5412: F, t1204: F, t1234: F, t1281: F, t1285: F, t1288: F, t1291: F, t1770: F, t1818: F, t1822: F, t1825: F, t3666: F, t3670: F, t3746: F, t3755: F, t460: F, t490: F, t5216: F, t5326: F, t5436: F, t5443: F, t5446: F, t5449: F, t5452: F, t5459: F, t5463: F, t5466: F, t5470: F, t5474: F, t5478: F, t1277: F, t1210: F, t1215: F, t1271: F, t1274: F, t1295: F, t1775: F, t1813: F, t1829: F, t3556: F, t3561: F, t3567: F, t3572: F, t3732: F, t495: F, t5220: F, t5225: F, t5231: F, t5237: F, t5246: F, t5251: F, t5414: F, t5417: F, t5423: F, t5429: F, t1832: F, t3801: F, t1298: F, t1300: F, t198: F, t336: F, t5023: F, t5062: F, t5065: F, t5067: F, t5070: F, t5107: F, t5111: F, t5189: F, t5191: F, t5194: F, t5196: F, t5200: F, t5204: F, t5209: F, t33: F, t265: F, t502: F, t4560: F, t1113: F, t1304: F, t1469: F, t1587: F, t1711: F, t1837: F, t4186: F, t4568: F, t504: F, t57: F, t606: F, t895: F, dens_threshold: F, rho1: F, zeta_threshold: F, t5035: F, t670: F, t93: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5481, t5486, t5487, t5491, t5494, t5497) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1090::<F>(t5332, t5480, t1811, t473, t1214, t1248, t1287, t489, t5412, t1204, t1234, t1281, t1285, t1288, t1291, t1770, t1818, t1822, t1825, t3666, t3670, t3746, t3755, t460, t490, t5216, t5326, t5436, t5443, t5446, t5449, t5452, t5459, t5463, t5466, t5470, t5474, t5478);
        let t5498 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1091::<F>(t1277, t5497);
        let t5501 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1092::<F>(t1204, t1210, t1215, t1271, t1274, t1295, t1770, t1775, t1813, t1829, t3556, t3561, t3567, t3572, t3732, t460, t495, t5216, t5220, t5225, t5231, t5237, t5246, t5251, t5414, t5417, t5423, t5429, t5498);
        let (t5505, t5508) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1093::<F>(t1832, t3801, t1298, t1300, t198, t336, t5023, t5062, t5065, t5067, t5070, t5107, t5111, t5189, t5191, t5194, t5196, t5200, t5204, t5209, t5501);
        let (t5509, t5516) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1094::<F>(t33, t265, t502, t4560, t5508, t1113, t1304, t1469, t1587, t1711, t1837, t4186, t4568, t504, t57, t606, t895, dens_threshold, rho1, zeta_threshold);
        let t5517 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1095::<F>(t5035, t5516);
        let t5523 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1096::<F>(t670, t93);
    (t5481, t5486, t5487, t5491, t5494, t5497, t5498, t5501, t5505, t5509, t5517, t5523)
}
