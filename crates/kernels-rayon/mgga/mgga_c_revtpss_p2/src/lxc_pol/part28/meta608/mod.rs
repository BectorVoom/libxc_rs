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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta608(t13821: f64, t27940: f64, t13928: f64, t26028: f64, t241: f64, t820: f64, t94491: f64, t13807: f64, t13817: f64, t13991: f64, t13793: f64, t13786: f64, t13854: f64, t5697: f64, t94429: f64, t5701: f64, t13995: f64, t98108: f64, t13977: f64, t27928: f64, t9775: f64, t13775: f64, t25986: f64, t2661: f64, t25978: f64, t5614: f64, t5622: f64, t94443: f64, t13769: f64, t240: f64, t7269: f64, t13756: f64, t7271: f64, t13760: f64, t25972: f64, t94424: f64, t94430: f64, t94444: f64, t94449: f64, t5609: f64, t7028: f64, t9845: f64, t1889: f64, t94545: f64, t13846: f64, t13877: f64, t7021: f64, t27932: f64, t48525: f64, t48141: f64, t5665: f64, t94497: f64, t13826: f64, t13923: f64, t7264: f64, t14036: f64, t25997: f64, t13946: f64, t94456: f64, t94460: f64, t13941: f64, t94423: f64, t14005: f64, t13834: f64, t13841: f64, t5706: f64, t1941: f64, t9817: f64, t48662: f64, t5651: f64, t9736: f64, t13985: f64, t13869: f64, t13878: f64, t94468: f64, t94472: f64, t94474: f64, t13967: f64, t13937: f64, t13981: f64, t94479: f64, t2689: f64, t27936: f64, t13857: f64, t94564: f64, t5629: f64, t1885: f64, t94459: f64, t26024: f64, t5661: f64, t14054: f64, t13874: f64, t94477: f64, t14046: f64, t14050: f64, t13850: f64, t2482: f64, t25981: f64, t814: f64, t13962: f64, t14020: f64, t7252: f64, t94484: f64, t94485: f64, t94498: f64, t94501: f64, t94503: f64, t94505: f64, t94509: f64, t94511: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t98110, t98112, t98116, t98118, t98120, t98122, t98124) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2107(t13821, t27940, t13928, t26028, t241, t820, t94491, t13807, t13817, t13991, t13793, t13786);
        let t98134 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2108(t13854, t26028, t5697, t94429, t5701, t13995, t98108, t98110, t98112, t98116, t98118, t98120, t98122, t98124);
        let (t98135, t98141, t98145, t98147, t98148, t98152) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2109(t13977, t26028, t27928, t9775, t13775, t25986, t2661, t25978, t5614, t5622, t94443, t13769, t240, t7269);
        let t98158 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2110(t13756, t7271, t13760, t25972, t94424, t94430, t94444, t94449, t98135, t98141, t98145, t98147, t98148, t98152);
        let (t98161, t98165, t98169, t98170, t98172, t98174) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2111(t5609, t7028, t9845, t1889, t94545, t13846, t13877, t7021, t27932, t48525, t48141, t5665, t94497);
        let t98184 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2112(t13826, t7271, t13923, t7264, t14036, t25997, t13946, t26028, t94456, t94460, t98161, t98165, t98169, t98170, t98172, t98174);
        let (t98186, t98188, t98189, t98191, t98194, t98197) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2113(t13941, t94423, t14005, t13834, t27940, t13841, t26028, t5706, t94429, t1941, t9817, t48662);
        let t98208 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2114(t5651, t7028, t9736, t13985, t94423, t13869, t7271, t13878, t25972, t94468, t94472, t94474, t98186, t98188, t98189, t98191, t98194, t98197);
        let (t98211, t98213, t98215, t98217, t98218, t98220, t98222, t98224) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2115(t13967, t26028, t13937, t13981, t94479, t2689, t27936, t13857, t94564, t25978, t5629, t1885, t94459);
        let t98233 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2116(t26024, t5661, t14054, t25986, t2661, t13874, t7271, t94477, t98211, t98213, t98215, t98217, t98218, t98220, t98222, t98224);
        let (t98236, t98239, t98244, t98245, t98253) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2117(t14046, t25986, t2661, t14050, t13850, t2482, t25981, t814, t13962, t26028, t14020, t7252);
        let t98255 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2118(t94484, t94485, t94498, t94501, t94503, t94505, t94509, t94511, t98236, t98239, t98244, t98245, t98253);
    (t98134, t98158, t98184, t98208, t98233, t98255)
}
