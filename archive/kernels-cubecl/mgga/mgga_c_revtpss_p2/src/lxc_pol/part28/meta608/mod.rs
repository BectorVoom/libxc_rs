//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta608 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2107;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2108;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2109;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2110;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2111;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2112;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2113;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2114;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2115;
use chunk9::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2116;
use chunk10::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2117;
use chunk11::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2118;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta608<F: Float>(t13821: F, t27940: F, t13928: F, t26028: F, t241: F, t820: F, t94491: F, t13807: F, t13817: F, t13991: F, t13793: F, t13786: F, t13854: F, t5697: F, t94429: F, t5701: F, t13995: F, t98108: F, t13977: F, t27928: F, t9775: F, t13775: F, t25986: F, t2661: F, t25978: F, t5614: F, t5622: F, t94443: F, t13769: F, t240: F, t7269: F, t13756: F, t7271: F, t13760: F, t25972: F, t94424: F, t94430: F, t94444: F, t94449: F, t5609: F, t7028: F, t9845: F, t1889: F, t94545: F, t13846: F, t13877: F, t7021: F, t27932: F, t48525: F, t48141: F, t5665: F, t94497: F, t13826: F, t13923: F, t7264: F, t14036: F, t25997: F, t13946: F, t94456: F, t94460: F, t13941: F, t94423: F, t14005: F, t13834: F, t13841: F, t5706: F, t1941: F, t9817: F, t48662: F, t5651: F, t9736: F, t13985: F, t13869: F, t13878: F, t94468: F, t94472: F, t94474: F, t13967: F, t13937: F, t13981: F, t94479: F, t2689: F, t27936: F, t13857: F, t94564: F, t5629: F, t1885: F, t94459: F, t26024: F, t5661: F, t14054: F, t13874: F, t94477: F, t14046: F, t14050: F, t13850: F, t2482: F, t25981: F, t814: F, t13962: F, t14020: F, t7252: F, t94484: F, t94485: F, t94498: F, t94501: F, t94503: F, t94505: F, t94509: F, t94511: F) -> (F, F, F, F, F, F) {
        let (t98110, t98112, t98116, t98118, t98120, t98122, t98124) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2107::<F>(t13821, t27940, t13928, t26028, t241, t820, t94491, t13807, t13817, t13991, t13793, t13786);
        let t98134 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2108::<F>(t13854, t26028, t5697, t94429, t5701, t13995, t98108, t98110, t98112, t98116, t98118, t98120, t98122, t98124);
        let (t98135, t98141, t98145, t98147, t98148, t98152) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2109::<F>(t13977, t26028, t27928, t9775, t13775, t25986, t2661, t25978, t5614, t5622, t94443, t13769, t240, t7269);
        let t98158 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2110::<F>(t13756, t7271, t13760, t25972, t94424, t94430, t94444, t94449, t98135, t98141, t98145, t98147, t98148, t98152);
        let (t98161, t98165, t98169, t98170, t98172, t98174) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2111::<F>(t5609, t7028, t9845, t1889, t94545, t13846, t13877, t7021, t27932, t48525, t48141, t5665, t94497);
        let t98184 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2112::<F>(t13826, t7271, t13923, t7264, t14036, t25997, t13946, t26028, t94456, t94460, t98161, t98165, t98169, t98170, t98172, t98174);
        let (t98186, t98188, t98189, t98191, t98194, t98197) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2113::<F>(t13941, t94423, t14005, t13834, t27940, t13841, t26028, t5706, t94429, t1941, t9817, t48662);
        let t98208 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2114::<F>(t5651, t7028, t9736, t13985, t94423, t13869, t7271, t13878, t25972, t94468, t94472, t94474, t98186, t98188, t98189, t98191, t98194, t98197);
        let (t98211, t98213, t98215, t98217, t98218, t98220, t98222, t98224) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2115::<F>(t13967, t26028, t13937, t13981, t94479, t2689, t27936, t13857, t94564, t25978, t5629, t1885, t94459);
        let t98233 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2116::<F>(t26024, t5661, t14054, t25986, t2661, t13874, t7271, t94477, t98211, t98213, t98215, t98217, t98218, t98220, t98222, t98224);
        let (t98236, t98239, t98244, t98245, t98253) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2117::<F>(t14046, t25986, t2661, t14050, t13850, t2482, t25981, t814, t13962, t26028, t14020, t7252);
        let t98255 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2118::<F>(t94484, t94485, t94498, t94501, t94503, t94505, t94509, t94511, t98236, t98239, t98244, t98245, t98253);
    (t98134, t98158, t98184, t98208, t98233, t98255)
}
