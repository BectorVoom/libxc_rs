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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta912<F: Float>(t52597: F, t52598: F, t63338: F, t63340: F, t63342: F, t63361: F, t63371: F, t77539: F, t77543: F, t77547: F, t77799: F, t52128: F, t52623: F, t63447: F, t63453: F, t63459: F, t77802: F, t77804: F, t77806: F, t77810: F, t77813: F, t77816: F, t77819: F, t63533: F, t63538: F, t63541: F, t63543: F, t63545: F, t63547: F, t63549: F, t63551: F, t77829: F, t77832: F, t77835: F, t77838: F, t41441: F, t63464: F, t77559: F, t77561: F, t77566: F, t77570: F, t77575: F, t77581: F, t77586: F, t77590: F, t77594: F, t77858: F, t51913: F, t51915: F, t63240: F, t63242: F, t77663: F, t77667: F, t77670: F, t77672: F, t77674: F, t77676: F, t77679: F, t41592: F, t77499: F, t77503: F, t77505: F, t77683: F, t77686: F, t77688: F, t77690: F, t77692: F, t77695: F, t77698: F, t77700: F, t41610: F, t63276: F, t63278: F, t77507: F, t77509: F, t77712: F, t77715: F, t77718: F, t77721: F, t77724: F, t77727: F, t77730: F, t41361: F, t51978: F, t52701: F, t63320: F, t77515: F, t77518: F, t77521: F, t77527: F, t77531: F, t77535: F, t77736: F, t77739: F, t52035: F, t52037: F, t52751: F, t915: F, t935: F, t23550: F, t41583: F, t23663: F, t914: F, t936: F, t23798: F, t945: F, t23811: F, t964: F, t41549: F, t52774: F, t52783: F, t52784: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t77935, t77947) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2932::<F>(t52597, t52598, t63338, t63340, t63342, t63361, t63371, t77539, t77543, t77547, t77799, t52128, t52623, t63447, t63453, t63459, t77802, t77804, t77806, t77810, t77813, t77816, t77819);
        let t77961 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2933::<F>(t63533, t63538, t63541, t63543, t63545, t63547, t63549, t63551, t77829, t77832, t77835, t77838);
        let t77974 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2934::<F>(t41441, t63464, t77559, t77561, t77566, t77570, t77575, t77581, t77586, t77590, t77594, t77858);
        let (t77998, t78010) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2935::<F>(t51913, t51915, t63240, t63242, t77663, t77667, t77670, t77672, t77674, t77676, t77679, t41592, t77499, t77503, t77505, t77683, t77686, t77688, t77690, t77692, t77695, t77698, t77700);
        let t78023 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2936::<F>(t41610, t63276, t63278, t77507, t77509, t77712, t77715, t77718, t77721, t77724, t77727, t77730);
        let t78035 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2937::<F>(t41361, t51978, t52701, t63320, t77515, t77518, t77521, t77527, t77531, t77535, t77736, t77739);
        let (t78049, t78061) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2938::<F>(t52035, t52037, t63338, t63340, t63342, t63361, t63371, t77539, t77543, t77547, t77799, t52128, t52751, t63447, t63453, t63459, t77802, t77804, t77806, t77810, t77813, t77816, t77819);
        let t78075 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2939::<F>(t63533, t63538, t63541, t63543, t63545, t63547, t63549, t63551, t77829, t77832, t77835, t77838);
        let t78088 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2940::<F>(t41441, t63464, t77559, t77561, t77566, t77570, t77575, t77581, t77586, t77590, t77594, t77858);
        let (t78094, t78096) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2941::<F>(t77998, t78010, t78023, t78035, t78049, t78061, t78075, t78088, t915, t935, t23550, t41583);
        let (t78099, t78108, t78111, t78132) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2942::<F>(t23663, t914, t936, t23798, t945, t23811, t964, t41361, t41549, t51978, t52774, t63276, t63278, t77499, t77503, t77505, t77507, t77509, t77515, t77518, t77521, t77527, t77531, t77535, t77539, t77543, t77547);
        let t78151 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2943::<F>(t52783, t52784, t63338, t63340, t63342, t63361, t63371, t63447, t63453, t63459, t63464, t77559, t77561, t77566, t77570, t77575, t77581, t77586, t77590, t77594);
    (t77935, t77947, t77961, t77974, t78094, t78096, t78099, t78108, t78111, t78132, t78151)
}
