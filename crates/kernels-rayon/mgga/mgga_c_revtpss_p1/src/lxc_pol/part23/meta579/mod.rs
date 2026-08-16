//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2190;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2191;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2192;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2193;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2194;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2195;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta579(t1559: f64, t18627: f64, t2747: f64, t18444: f64, t6035: f64, t10770: f64, t18469: f64, t1544: f64, t2723: f64, t18426: f64, t14846: f64, t14850: f64, t14866: f64, t18403: f64, t18411: f64, t18416: f64, t18420: f64, t18424: f64, t18433: f64, t18442: f64, t2745: f64, t4362: f64, t10698: f64, t23114: f64, t828: f64, t23148: f64, t855: f64, t10824: f64, t10826: f64, t10885: f64, t18459: f64, t18475: f64, t18485: f64, t18487: f64, t18491: f64, t18518: f64, t18532: f64, t18623: f64, t18644: f64, t851: f64, t23278: f64, t23310: f64, t10645: f64, t10651: f64, t10952: f64, t14512: f64, t14525: f64, t14533: f64, t14558: f64, t14564: f64, t18690: f64, t18699: f64, t213: f64, t23160: f64, t23168: f64, t23172: f64, t23177: f64, t23245: f64, t234: f64, t2811: f64, t4494: f64, t4504: f64, t4514: f64, t4526: f64, t5978: f64, t6017: f64, t820: f64, t879: f64, t10939: f64, t10948: f64, t10969: f64, t10971: f64, t14581: f64, t14948: f64, t14951: f64, t14961: f64, t18714: f64, t18720: f64, t18727: f64, t18731: f64, t18733: f64, t18739: f64, t18743: f64, t18747: f64, t18751: f64, t18763: f64, t6022: f64, t868: f64, t225: f64, t10501: f64, t10503: f64, t10984: f64, t14474: f64, t14486: f64, t14998: f64, t15004: f64, t15006: f64, t15015: f64, t18318: f64, t257: f64, t4474: f64, t6049: f64, t6072: f64, t865: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23323, t23327, t23331, t23334, t23336, t23339) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2190(t1559, t18627, t2747, t18444, t6035, t10770, t18469, t1544, t2723, t18426, t14846, t14850, t14866, t18403, t18411, t18416, t18420, t18424, t18433, t18442, t2745, t4362);
        let (t23342, t23346, t23357) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2191(t10698, t23114, t828, t23148, t855, t10824, t10826, t10885, t18459, t18475, t18485, t18487, t18491, t18518, t18532, t18623, t18644, t851);
        let t23359 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2192(t23278, t23310, t23339, t23357);
        let t23363 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2193(t10645, t10651, t10952, t14512, t14525, t14533, t14558, t14564, t1559, t18690, t18699, t213, t23160, t23168, t23172, t23177, t23245, t23359, t234, t2811, t4494, t4504, t4514, t4526, t5978, t6017, t820, t879);
        let t23382 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2194(t10939, t10948, t10969, t10971, t14581, t14948, t14951, t14961, t1559, t18714, t18720, t18727, t18731, t18733, t18739, t18743, t18747, t18751, t18763, t6022, t820);
        let (t23383, t23384, t23388, t23400) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2195(t23363, t23382, t868, t225, t23359, t10501, t10503, t10984, t14474, t14486, t14998, t15004, t15006, t15015, t18318, t213, t257, t4474, t6049, t6072, t865);
    (t23323, t23327, t23331, t23334, t23336, t23342, t23346, t23359, t23383, t23384, t23388, t23400)
}
