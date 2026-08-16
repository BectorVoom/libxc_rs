//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta912 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2932;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2933;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2934;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2935;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2936;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2937;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2938;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2939;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2940;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2941;
use chunk10::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2942;
use chunk11::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2943;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta912(t52597: f64, t52598: f64, t63338: f64, t63340: f64, t63342: f64, t63361: f64, t63371: f64, t77539: f64, t77543: f64, t77547: f64, t77799: f64, t52128: f64, t52623: f64, t63447: f64, t63453: f64, t63459: f64, t77802: f64, t77804: f64, t77806: f64, t77810: f64, t77813: f64, t77816: f64, t77819: f64, t63533: f64, t63538: f64, t63541: f64, t63543: f64, t63545: f64, t63547: f64, t63549: f64, t63551: f64, t77829: f64, t77832: f64, t77835: f64, t77838: f64, t41441: f64, t63464: f64, t77559: f64, t77561: f64, t77566: f64, t77570: f64, t77575: f64, t77581: f64, t77586: f64, t77590: f64, t77594: f64, t77858: f64, t51913: f64, t51915: f64, t63240: f64, t63242: f64, t77663: f64, t77667: f64, t77670: f64, t77672: f64, t77674: f64, t77676: f64, t77679: f64, t41592: f64, t77499: f64, t77503: f64, t77505: f64, t77683: f64, t77686: f64, t77688: f64, t77690: f64, t77692: f64, t77695: f64, t77698: f64, t77700: f64, t41610: f64, t63276: f64, t63278: f64, t77507: f64, t77509: f64, t77712: f64, t77715: f64, t77718: f64, t77721: f64, t77724: f64, t77727: f64, t77730: f64, t41361: f64, t51978: f64, t52701: f64, t63320: f64, t77515: f64, t77518: f64, t77521: f64, t77527: f64, t77531: f64, t77535: f64, t77736: f64, t77739: f64, t52035: f64, t52037: f64, t52751: f64, t915: f64, t935: f64, t23550: f64, t41583: f64, t23663: f64, t914: f64, t936: f64, t23798: f64, t945: f64, t23811: f64, t964: f64, t41549: f64, t52774: f64, t52783: f64, t52784: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t77935, t77947) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2932(t52597, t52598, t63338, t63340, t63342, t63361, t63371, t77539, t77543, t77547, t77799, t52128, t52623, t63447, t63453, t63459, t77802, t77804, t77806, t77810, t77813, t77816, t77819);
        let t77961 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2933(t63533, t63538, t63541, t63543, t63545, t63547, t63549, t63551, t77829, t77832, t77835, t77838);
        let t77974 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2934(t41441, t63464, t77559, t77561, t77566, t77570, t77575, t77581, t77586, t77590, t77594, t77858);
        let (t77998, t78010) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2935(t51913, t51915, t63240, t63242, t77663, t77667, t77670, t77672, t77674, t77676, t77679, t41592, t77499, t77503, t77505, t77683, t77686, t77688, t77690, t77692, t77695, t77698, t77700);
        let t78023 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2936(t41610, t63276, t63278, t77507, t77509, t77712, t77715, t77718, t77721, t77724, t77727, t77730);
        let t78035 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2937(t41361, t51978, t52701, t63320, t77515, t77518, t77521, t77527, t77531, t77535, t77736, t77739);
        let (t78049, t78061) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2938(t52035, t52037, t63338, t63340, t63342, t63361, t63371, t77539, t77543, t77547, t77799, t52128, t52751, t63447, t63453, t63459, t77802, t77804, t77806, t77810, t77813, t77816, t77819);
        let t78075 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2939(t63533, t63538, t63541, t63543, t63545, t63547, t63549, t63551, t77829, t77832, t77835, t77838);
        let t78088 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2940(t41441, t63464, t77559, t77561, t77566, t77570, t77575, t77581, t77586, t77590, t77594, t77858);
        let (t78094, t78096) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2941(t77998, t78010, t78023, t78035, t78049, t78061, t78075, t78088, t915, t935, t23550, t41583);
        let (t78099, t78108, t78111, t78132) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2942(t23663, t914, t936, t23798, t945, t23811, t964, t41361, t41549, t51978, t52774, t63276, t63278, t77499, t77503, t77505, t77507, t77509, t77515, t77518, t77521, t77527, t77531, t77535, t77539, t77543, t77547);
        let t78151 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2943(t52783, t52784, t63338, t63340, t63342, t63361, t63371, t63447, t63453, t63459, t63464, t77559, t77561, t77566, t77570, t77575, t77581, t77586, t77590, t77594);
    (t77935, t77947, t77961, t77974, t78094, t78096, t78099, t78108, t78111, t78132, t78151)
}
