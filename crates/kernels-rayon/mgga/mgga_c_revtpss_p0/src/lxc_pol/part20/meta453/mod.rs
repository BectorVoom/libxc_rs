//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1729;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1730;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1731;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1732;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1733;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1734;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1735;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1736;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta453(t2452: f64, t40633: f64, t46808: f64, t547: f64, t9793: f64, t9794: f64, t9930: f64, t268: f64, t40634: f64, t550: f64, t9718: f64, t247: f64, t548: f64, t9722: f64, t1379: f64, t40846: f64, t816: f64, t1412: f64, t1353: f64, t1399: f64, t40609: f64, t4062: f64, t3994: f64, t40763: f64, t2735: f64, t9792: f64, t1413: f64, t1376: f64, t40769: f64, t3989: f64, t9986: f64, t1410: f64, t4012: f64, t46787: f64, t46789: f64, t46793: f64, t46797: f64, t46800: f64, t46804: f64, t828: f64, t9628: f64, t10001: f64, t221: f64, t4019: f64, t9912: f64, t10111: f64, t1386: f64, t9720: f64, t1390: f64, t685: f64, t9970: f64, t9976: f64, t3930: f64, t9893: f64, t3957: f64, t9700: f64, t807: f64, t3952: f64, t9784: f64, t281: f64, t39644: f64, t40650: f64, t2689: f64, t9715: f64, t40688: f64, t46786: f64, t9400: f64, t9941: f64, t9704: f64, t4003: f64, t46531: f64, t124: f64, t1370: f64, t13789: f64, t13791: f64, t3829: f64, t3889: f64, t3934: f64, t3938: f64, t4002: f64, t46345: f64, t46432: f64, t5671: f64, t800: f64, t9840: f64, t9942: f64, t2682: f64, t820: f64, t3940: f64, t3960: f64, t9816: f64, t9818: f64, t5744: f64, t808: f64, t9935: f64, t9845: f64, t9769: f64, t2713: f64, t3964: f64, t9703: f64, t4086: f64, t9801: f64, t9846: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46810, t46812, t46817, t46820) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1729(t2452, t40633, t46808, t547, t9793, t9794, t9930, t268, t40634, t550, t9718, t247, t548, t9722);
        let (t46824, t46826, t46828, t46831, t46833) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1730(t1379, t40846, t550, t816, t1412, t9794, t1353, t1399, t9793, t40609, t4062, t3994, t40763);
        let t46848 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1731(t2735, t9792, t1413, t46826, t1376, t40769, t3989, t9986, t1353, t1410, t4012, t46787, t46789, t46793, t46797, t46800, t46804, t46810, t46812, t46817, t46820, t46824, t46828, t46831, t46833, t828, t9628);
        let (t46853, t46859, t46861) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1732(t10001, t221, t4019, t9912, t10111, t1386, t9720, t1390, t1399, t685, t9970, t9976);
        let (t46863, t46865, t46877, t46879, t46885) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1733(t3930, t9893, t3957, t9700, t1413, t547, t807, t9628, t3952, t9784, t281, t39644, t40650, t550);
        let (t46902, t46911) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1734(t2689, t9715, t40688, t547, t46786, t807, t9400, t9941, t9704, t4003, t46531, t124, t1370, t13789, t13791, t1390, t1410, t3829, t3889, t3934, t3938, t4002, t46345, t46432, t46853, t46859, t46861, t46863, t46865, t46877, t46879, t46885, t5671, t800, t828, t9840, t9942);
        let (t46918, t46922, t46924, t46929) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1735(t1386, t2682, t820, t3940, t1399, t3960, t9816, t9818, t3829, t4003, t2735, t5744);
        let (t46931, t46934, t46941, t46944, t46947) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1736(t46929, t808, t9935, t9845, t9930, t9769, t2713, t3964, t9703, t4086, t9801, t9846);
    (t46848, t46902, t46911, t46918, t46922, t46924, t46931, t46934, t46941, t46944, t46947)
}
