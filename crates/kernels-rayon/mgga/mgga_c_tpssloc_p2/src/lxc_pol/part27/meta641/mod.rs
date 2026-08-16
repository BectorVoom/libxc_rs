//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta641 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2173;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2174;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2175;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2176;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2177;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2178;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2179;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2180;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2181;
use chunk9::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2182;
use chunk10::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2183;
use chunk11::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta641(t87618: f64, t1902: f64, t4233: f64, t1888: f64, t232: f64, t47528: f64, t6646: f64, t13398: f64, t82018: f64, t13404: f64, t22996: f64, t7521: f64, t81632: f64, t23035: f64, t2379: f64, t25319: f64, t6637: f64, t1887: f64, t81959: f64, t25248: f64, t25249: f64, t4265: f64, t828: f64, t22690: f64, t23171: f64, t2613: f64, t4291: f64, t7535: f64, t81697: f64, t81704: f64, t81717: f64, t829: f64, t87609: f64, t87613: f64, t87615: f64, t2632: f64, t87106: f64, t23143: f64, t7525: f64, t25238: f64, t6579: f64, t23153: f64, t4119: f64, t6552: f64, t12971: f64, t6638: f64, t22893: f64, t23164: f64, t25312: f64, t82011: f64, t47425: f64, t13336: f64, t1909: f64, t25269: f64, t2617: f64, t4182: f64, t4281: f64, t7533: f64, t81980: f64, t81989: f64, t82005: f64, t82013: f64, t82016: f64, t9612: f64, t25038: f64, t776: f64, t87130: f64, t22986: f64, t87111: f64, t82039: f64, t25273: f64, t244: f64, t268: f64, t6559: f64, t25250: f64, t87202: f64, t25316: f64, t82038: f64, t47439: f64, t23110: f64, t23185: f64, t25272: f64, t25325: f64, t6547: f64, t13390: f64, t23016: f64, t25255: f64, t25262: f64, t25295: f64, t2679: f64, t2684: f64, t4162: f64, t4166: f64, t6660: f64, t808: f64, t812: f64, t82028: f64, t82032: f64, t82047: f64, t1912: f64, t46452: f64, t82143: f64, t82145: f64, t82150: f64, t855: f64, t858: f64, t87029: f64, t87033: f64, t87039: f64, t87042: f64, t87047: f64, t87050: f64, t87094: f64, t87146: f64, t87524: f64, t87562: f64, t87606: f64, t1880: f64, t7488: f64, t82124: f64, t1911: f64, t40889: f64, t25045: f64, t82074: f64, t254: f64, t799: f64, t225: f64, t25161: f64, t23270: f64, t25039: f64, t23218: f64, t25224: f64, t6562: f64, t6572: f64, t86893: f64, t23228: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87619, t87620, t87627, t87630, t87633, t87635) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2173(t87618, t1902, t4233, t1888, t232, t47528, t6646, t13398, t82018, t13404, t22996, t7521, t81632);
        let (t87640, t87642, t87645, t87650) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2174(t23035, t2379, t25319, t6637, t1887, t81959, t25248, t25249, t1888, t232, t4265, t6646, t828);
        let t87656 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2175(t22690, t23171, t25319, t2613, t4291, t7535, t81697, t81704, t81717, t829, t87609, t87613, t87615, t87619, t87620, t87627, t87630, t87633, t87635, t87640, t87645, t87650);
        let (t87660, t87666, t87669, t87672) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2176(t1888, t22996, t2632, t87106, t23143, t7525, t25238, t6579, t23153, t4119, t6552, t6637);
        let (t87676, t87680, t87687, t87692) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2177(t12971, t6552, t6637, t6638, t22893, t23164, t25312, t82011, t1888, t232, t47425, t6646);
        let t87694 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2178(t13336, t1909, t25269, t2617, t4182, t4281, t7533, t81980, t81989, t82005, t82013, t82016, t87620, t87660, t87666, t87669, t87672, t87676, t87680, t87687, t87692, t9612);
        let (t87699, t87705, t87708, t87710, t87712) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2179(t25038, t25248, t776, t87130, t22986, t6646, t829, t87111, t82039, t25273, t6579, t244, t268, t6559);
        let (t87714, t87718, t87726, t87729) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2180(t25250, t87202, t87712, t25316, t82038, t1888, t232, t47439, t6646, t23110, t23185, t25272);
        let t87735 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2181(t87729, t25325, t6547, t13390, t23016, t25255, t25262, t25295, t2679, t2684, t4162, t4166, t6660, t808, t812, t82028, t82032, t82047, t87699, t87705, t87708, t87710, t87714, t87718, t87726);
        let t87741 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2182(t1912, t46452, t82143, t82145, t82150, t855, t858, t87029, t87033, t87039, t87042, t87047, t87050, t87094, t87146, t87524, t87562, t87606, t87656, t87694, t87735);
        let (t87746, t87748, t87754, t87755, t87758) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2183(t1880, t7488, t82124, t1911, t40889, t23185, t25045, t82074, t254, t799, t225, t25161);
        let (t87765, t87773, t87777, t87779) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2184(t23270, t2379, t25039, t87642, t1880, t23218, t25224, t6562, t6572, t86893, t23171, t23228, t7488);
    (t87712, t87741, t87746, t87748, t87754, t87755, t87758, t87765, t87773, t87777, t87779)
}
