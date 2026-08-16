//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta408 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1429;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1430;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1431;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1432;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1433;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1434;
use chunk6::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1435;
use chunk7::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1436;
use chunk8::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1437;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta408(t3957: f64, t6884: f64, t124: f64, t21969: f64, t800: f64, t6850: f64, t9744: f64, t125: f64, t6861: f64, t3936: f64, t9835: f64, t1414: f64, t828: f64, t221: f64, t3979: f64, t6816: f64, t3978: f64, t3989: f64, t6880: f64, t22025: f64, t543: f64, t3992: f64, t2661: f64, t1370: f64, t13779: f64, t13781: f64, t13797: f64, t1410: f64, t5671: f64, t9735: f64, t6836: f64, t9921: f64, t1399: f64, t6843: f64, t3938: f64, t5673: f64, t21990: f64, t5674: f64, t13944: f64, t6869: f64, t5591: f64, t13848: f64, t9818: f64, t9816: f64, t13798: f64, t13801: f64, t13810: f64, t13813: f64, t3934: f64, t5659: f64, t9955: f64, t1413: f64, t547: f64, t807: f64, t4011: f64, t1353: f64, t6883: f64, t13832: f64, t13851: f64, t13858: f64, t3944: f64, t9739: f64, t9742: f64, t9766: f64, t13784: f64, t13790: f64, t13789: f64, t13880: f64, t13943: f64, t13949: f64, t13954: f64, t13956: f64, t9776: f64, t9780: f64, t9786: f64, t9791: f64, t9796: f64, t9799: f64, t6871: f64, t9962: f64, t22016: f64, t5675: f64, t6849: f64, t1872: f64, t13804: f64, t13959: f64, t13987: f64, t13988: f64, t14001: f64, t14007: f64, t9748: f64, t9804: f64, t9847: f64, t9910: f64, t3930: f64, t6846: f64, t4019: f64, t6862: f64, t10001: f64, t6800: f64, t72: f64, t757: f64, t1317: f64, t6801: f64, t13599: f64, t21901: f64, t21905: f64, t21933: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9374: f64, t9389: f64, t9391: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22038, t22041, t22044, t22046, t22048, t22052) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1429(t3957, t6884, t124, t21969, t800, t6850, t9744, t125, t6861, t3936, t9835, t1414, t828);
        let t22065 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1430(t221, t3979, t6816, t3978, t3989, t6880, t22025, t543, t3992, t2661, t1370, t13779, t13781, t13797, t1410, t22038, t22041, t22044, t22048, t22052, t5671, t9735);
        let (t22069, t22076, t22079, t22081, t22085, t22089) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1431(t221, t6836, t9921, t3978, t125, t6816, t1399, t3936, t6843, t3938, t5673, t21990, t5674);
        let t22105 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1432(t13944, t3936, t6869, t543, t5591, t5674, t13848, t9818, t9816, t13798, t13801, t13810, t13813, t22069, t22076, t22081, t22085, t22089, t3934, t5671);
        let (t22107, t22111, t22115, t22120, t22125) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1433(t22046, t3936, t3938, t5659, t5673, t5674, t1399, t125, t6836, t9955, t1413, t6816);
        let t22140 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1434(t22125, t547, t807, t4011, t6836, t1353, t6883, t800, t13832, t13851, t13858, t22107, t22111, t22115, t22120, t3934, t3944, t9739, t9742, t9766);
        let t22153 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1435(t13784, t13790, t13789, t13880, t13943, t13949, t13954, t13956, t5671, t9776, t9780, t9786, t9791, t9796, t9799);
        let t22176 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1436(t6871, t9962, t22016, t22046, t5673, t5675, t1353, t6849, t800, t1872, t5591, t13804, t13959, t13987, t13988, t14001, t14007, t3944, t5671, t9748, t9804, t9847, t9910);
        let (t22179, t22183, t22187, t22189, t22190) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1437(t3930, t6846, t221, t4019, t6862, t10001, t6800, t72, t757, t1317, t6801, t13599, t21901, t21905, t21933, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
    (t22065, t22079, t22105, t22140, t22153, t22176, t22179, t22183, t22187, t22189, t22190)
}
