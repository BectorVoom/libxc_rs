//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta194 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1169;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1170;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1171;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1172;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1173;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1174;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1175;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta194(t1214: f64, t5486: f64, t1248: f64, t1287: f64, t1811: f64, t489: f64, t5412: f64, t1204: f64, t1234: f64, t1281: f64, t1285: f64, t1288: f64, t1291: f64, t1770: f64, t1818: f64, t1822: f64, t1825: f64, t3666: f64, t3670: f64, t3746: f64, t3755: f64, t460: f64, t490: f64, t5216: f64, t5326: f64, t5436: f64, t5443: f64, t5446: f64, t5449: f64, t5452: f64, t5459: f64, t5463: f64, t5466: f64, t5470: f64, t5474: f64, t5478: f64, t5481: f64, t1277: f64, t1210: f64, t1215: f64, t1271: f64, t1274: f64, t1295: f64, t1775: f64, t1813: f64, t1829: f64, t3556: f64, t3561: f64, t3567: f64, t3572: f64, t3732: f64, t495: f64, t5220: f64, t5225: f64, t5231: f64, t5237: f64, t5246: f64, t5251: f64, t5414: f64, t5417: f64, t5423: f64, t5429: f64, t1832: f64, t3801: f64, t1298: f64, t1300: f64, t198: f64, t336: f64, t5023: f64, t5062: f64, t5065: f64, t5067: f64, t5070: f64, t5107: f64, t5111: f64, t5189: f64, t5191: f64, t5194: f64, t5196: f64, t5200: f64, t5204: f64, t5209: f64, t33: f64, t265: f64, t502: f64, t4560: f64, t1113: f64, t1304: f64, t1469: f64, t1587: f64, t1711: f64, t1837: f64, t4186: f64, t4568: f64, t504: f64, t57: f64, t606: f64, t895: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t5035: f64, t670: f64, t93: f64, t1312: f64, t1518: f64, t2322: f64, t4246: f64, t4248: f64, t4292: f64, t1450: f64, t1907: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5487, t5491, t5494, t5497) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1169(t1214, t5486, t1248, t1287, t1811, t489, t5412, t1204, t1234, t1281, t1285, t1288, t1291, t1770, t1818, t1822, t1825, t3666, t3670, t3746, t3755, t460, t490, t5216, t5326, t5436, t5443, t5446, t5449, t5452, t5459, t5463, t5466, t5470, t5474, t5478, t5481);
        let t5498 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1170(t1277, t5497);
        let t5501 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1171(t1204, t1210, t1215, t1271, t1274, t1295, t1770, t1775, t1813, t1829, t3556, t3561, t3567, t3572, t3732, t460, t495, t5216, t5220, t5225, t5231, t5237, t5246, t5251, t5414, t5417, t5423, t5429, t5498);
        let (t5505, t5508) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1172(t1832, t3801, t1298, t1300, t198, t336, t5023, t5062, t5065, t5067, t5070, t5107, t5111, t5189, t5191, t5194, t5196, t5200, t5204, t5209, t5501);
        let (t5509, t5516) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1173(t33, t265, t502, t4560, t5508, t1113, t1304, t1469, t1587, t1711, t1837, t4186, t4568, t504, t57, t606, t895, dens_threshold, rho1, zeta_threshold);
        let t5517 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1174(t5035, t5516);
        let (t5523, t5528, t5532) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1175(t670, t93, t1312, t1518, t2322, t4246, t4248, t4292, t1450, t1907);
    (t5487, t5491, t5494, t5497, t5498, t5501, t5505, t5509, t5517, t5523, t5528, t5532)
}
