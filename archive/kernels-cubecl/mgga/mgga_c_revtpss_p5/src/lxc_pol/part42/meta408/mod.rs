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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta408<F: Float>(t3957: F, t6884: F, t124: F, t21969: F, t800: F, t6850: F, t9744: F, t125: F, t6861: F, t3936: F, t9835: F, t1414: F, t828: F, t221: F, t3979: F, t6816: F, t3978: F, t3989: F, t6880: F, t22025: F, t543: F, t3992: F, t2661: F, t1370: F, t13779: F, t13781: F, t13797: F, t1410: F, t5671: F, t9735: F, t6836: F, t9921: F, t1399: F, t6843: F, t3938: F, t5673: F, t21990: F, t5674: F, t13944: F, t6869: F, t5591: F, t13848: F, t9818: F, t9816: F, t13798: F, t13801: F, t13810: F, t13813: F, t3934: F, t5659: F, t9955: F, t1413: F, t547: F, t807: F, t4011: F, t1353: F, t6883: F, t13832: F, t13851: F, t13858: F, t3944: F, t9739: F, t9742: F, t9766: F, t13784: F, t13790: F, t13789: F, t13880: F, t13943: F, t13949: F, t13954: F, t13956: F, t9776: F, t9780: F, t9786: F, t9791: F, t9796: F, t9799: F, t6871: F, t9962: F, t22016: F, t5675: F, t6849: F, t1872: F, t13804: F, t13959: F, t13987: F, t13988: F, t14001: F, t14007: F, t9748: F, t9804: F, t9847: F, t9910: F, t3930: F, t6846: F, t4019: F, t6862: F, t10001: F, t6800: F, t72: F, t757: F, t1317: F, t6801: F, t13599: F, t21901: F, t21905: F, t21933: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22038, t22041, t22044, t22046, t22048, t22052) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1429::<F>(t3957, t6884, t124, t21969, t800, t6850, t9744, t125, t6861, t3936, t9835, t1414, t828);
        let t22065 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1430::<F>(t221, t3979, t6816, t3978, t3989, t6880, t22025, t543, t3992, t2661, t1370, t13779, t13781, t13797, t1410, t22038, t22041, t22044, t22048, t22052, t5671, t9735);
        let (t22069, t22076, t22079, t22081, t22085, t22089) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1431::<F>(t221, t6836, t9921, t3978, t125, t6816, t1399, t3936, t6843, t3938, t5673, t21990, t5674);
        let t22105 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1432::<F>(t13944, t3936, t6869, t543, t5591, t5674, t13848, t9818, t9816, t13798, t13801, t13810, t13813, t22069, t22076, t22081, t22085, t22089, t3934, t5671);
        let (t22107, t22111, t22115, t22120, t22125) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1433::<F>(t22046, t3936, t3938, t5659, t5673, t5674, t1399, t125, t6836, t9955, t1413, t6816);
        let t22140 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1434::<F>(t22125, t547, t807, t4011, t6836, t1353, t6883, t800, t13832, t13851, t13858, t22107, t22111, t22115, t22120, t3934, t3944, t9739, t9742, t9766);
        let t22153 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1435::<F>(t13784, t13790, t13789, t13880, t13943, t13949, t13954, t13956, t5671, t9776, t9780, t9786, t9791, t9796, t9799);
        let t22176 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1436::<F>(t6871, t9962, t22016, t22046, t5673, t5675, t1353, t6849, t800, t1872, t5591, t13804, t13959, t13987, t13988, t14001, t14007, t3944, t5671, t9748, t9804, t9847, t9910);
        let (t22179, t22183, t22187, t22189, t22190) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1437::<F>(t3930, t6846, t221, t4019, t6862, t10001, t6800, t72, t757, t1317, t6801, t13599, t21901, t21905, t21933, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
    (t22065, t22079, t22105, t22140, t22153, t22176, t22179, t22183, t22187, t22189, t22190)
}
