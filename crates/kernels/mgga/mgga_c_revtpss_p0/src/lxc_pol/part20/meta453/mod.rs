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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1729;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1730;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1731;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1732;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1733;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1734;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1735;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1736;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta453<F: Float>(t2452: F, t40633: F, t46808: F, t547: F, t9793: F, t9794: F, t9930: F, t268: F, t40634: F, t550: F, t9718: F, t247: F, t548: F, t9722: F, t1379: F, t40846: F, t816: F, t1412: F, t1353: F, t1399: F, t40609: F, t4062: F, t3994: F, t40763: F, t2735: F, t9792: F, t1413: F, t1376: F, t40769: F, t3989: F, t9986: F, t1410: F, t4012: F, t46787: F, t46789: F, t46793: F, t46797: F, t46800: F, t46804: F, t828: F, t9628: F, t10001: F, t221: F, t4019: F, t9912: F, t10111: F, t1386: F, t9720: F, t1390: F, t685: F, t9970: F, t9976: F, t3930: F, t9893: F, t3957: F, t9700: F, t807: F, t3952: F, t9784: F, t281: F, t39644: F, t40650: F, t2689: F, t9715: F, t40688: F, t46786: F, t9400: F, t9941: F, t9704: F, t4003: F, t46531: F, t124: F, t1370: F, t13789: F, t13791: F, t3829: F, t3889: F, t3934: F, t3938: F, t4002: F, t46345: F, t46432: F, t5671: F, t800: F, t9840: F, t9942: F, t2682: F, t820: F, t3940: F, t3960: F, t9816: F, t9818: F, t5744: F, t808: F, t9935: F, t9845: F, t9769: F, t2713: F, t3964: F, t9703: F, t4086: F, t9801: F, t9846: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t46810, t46812, t46817, t46820) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1729::<F>(t2452, t40633, t46808, t547, t9793, t9794, t9930, t268, t40634, t550, t9718, t247, t548, t9722);
        let (t46824, t46826, t46828, t46831, t46833) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1730::<F>(t1379, t40846, t550, t816, t1412, t9794, t1353, t1399, t9793, t40609, t4062, t3994, t40763);
        let t46848 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1731::<F>(t2735, t9792, t1413, t46826, t1376, t40769, t3989, t9986, t1353, t1410, t4012, t46787, t46789, t46793, t46797, t46800, t46804, t46810, t46812, t46817, t46820, t46824, t46828, t46831, t46833, t828, t9628);
        let (t46853, t46859, t46861) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1732::<F>(t10001, t221, t4019, t9912, t10111, t1386, t9720, t1390, t1399, t685, t9970, t9976);
        let (t46863, t46865, t46877, t46879, t46885) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1733::<F>(t3930, t9893, t3957, t9700, t1413, t547, t807, t9628, t3952, t9784, t281, t39644, t40650, t550);
        let (t46902, t46911) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1734::<F>(t2689, t9715, t40688, t547, t46786, t807, t9400, t9941, t9704, t4003, t46531, t124, t1370, t13789, t13791, t1390, t1410, t3829, t3889, t3934, t3938, t4002, t46345, t46432, t46853, t46859, t46861, t46863, t46865, t46877, t46879, t46885, t5671, t800, t828, t9840, t9942);
        let (t46918, t46922, t46924, t46929) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1735::<F>(t1386, t2682, t820, t3940, t1399, t3960, t9816, t9818, t3829, t4003, t2735, t5744);
        let (t46931, t46934, t46941, t46944, t46947) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1736::<F>(t46929, t808, t9935, t9845, t9930, t9769, t2713, t3964, t9703, t4086, t9801, t9846);
    (t46848, t46902, t46911, t46918, t46922, t46924, t46931, t46934, t46941, t46944, t46947)
}
