//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta335 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1167;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1168;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1169;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1170;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1171;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1172;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta335<F: Float>(t1559: F, t18627: F, t2747: F, t18444: F, t6035: F, t10770: F, t18469: F, t1544: F, t2723: F, t18426: F, t14846: F, t14850: F, t14866: F, t18403: F, t18411: F, t18416: F, t18420: F, t18424: F, t18433: F, t18442: F, t2745: F, t4362: F, t10698: F, t23114: F, t828: F, t23148: F, t855: F, t10824: F, t10826: F, t10885: F, t18459: F, t18475: F, t18485: F, t18487: F, t18491: F, t18518: F, t18532: F, t18623: F, t18644: F, t851: F, t23278: F, t23310: F, t10645: F, t10651: F, t10952: F, t14512: F, t14525: F, t14533: F, t14558: F, t14564: F, t18690: F, t18699: F, t213: F, t23160: F, t23168: F, t23172: F, t23177: F, t23245: F, t234: F, t2811: F, t4494: F, t4504: F, t4514: F, t4526: F, t5978: F, t6017: F, t820: F, t879: F, t10939: F, t10948: F, t10969: F, t10971: F, t14581: F, t14948: F, t14951: F, t14961: F, t18714: F, t18720: F, t18727: F, t18731: F, t18733: F, t18739: F, t18743: F, t18747: F, t18751: F, t18763: F, t6022: F, t868: F, t225: F, t10501: F, t10503: F, t10984: F, t14474: F, t14486: F, t14998: F, t15004: F, t15006: F, t15015: F, t18318: F, t257: F, t4474: F, t6049: F, t6072: F, t865: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23323, t23327, t23331, t23334, t23336, t23339) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1167::<F>(t1559, t18627, t2747, t18444, t6035, t10770, t18469, t1544, t2723, t18426, t14846, t14850, t14866, t18403, t18411, t18416, t18420, t18424, t18433, t18442, t2745, t4362);
        let (t23342, t23346, t23357) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1168::<F>(t10698, t23114, t828, t23148, t855, t10824, t10826, t10885, t18459, t18475, t18485, t18487, t18491, t18518, t18532, t18623, t18644, t851);
        let t23359 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1169::<F>(t23278, t23310, t23339, t23357);
        let t23363 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1170::<F>(t10645, t10651, t10952, t14512, t14525, t14533, t14558, t14564, t1559, t18690, t18699, t213, t23160, t23168, t23172, t23177, t23245, t23359, t234, t2811, t4494, t4504, t4514, t4526, t5978, t6017, t820, t879);
        let t23382 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1171::<F>(t10939, t10948, t10969, t10971, t14581, t14948, t14951, t14961, t1559, t18714, t18720, t18727, t18731, t18733, t18739, t18743, t18747, t18751, t18763, t6022, t820);
        let (t23383, t23384, t23388, t23400) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1172::<F>(t23363, t23382, t868, t225, t23359, t10501, t10503, t10984, t14474, t14486, t14998, t15004, t15006, t15015, t18318, t213, t257, t4474, t6049, t6072, t865);
    (t23323, t23327, t23331, t23334, t23336, t23342, t23346, t23359, t23383, t23384, t23388, t23400)
}
