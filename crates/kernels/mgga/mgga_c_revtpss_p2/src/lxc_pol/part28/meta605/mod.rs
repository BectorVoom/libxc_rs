//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta605 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2090;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2091;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2092;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2093;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2094;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2095;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2096;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2097;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta605<F: Float>(t2435: F, t27986: F, t1904: F, t2439: F, t25916: F, t26050: F, t27884: F, t25304: F, t27883: F, t25946: F, t25898: F, t97699: F, t25901: F, t1364: F, t27961: F, t786: F, t2453: F, t3908: F, t7911: F, t136: F, t2457: F, t7920: F, t94589: F, t27965: F, t14090: F, t26054: F, t14268: F, t2022: F, t7295: F, t7296: F, t7921: F, t94608: F, t94610: F, t94613: F, t94616: F, t25894: F, t97676: F, t97680: F, t1444: F, t5659: F, t14110: F, t94901: F, t10073: F, t1903: F, t2029: F, t25929: F, t25930: F, t25931: F, t27868: F, t49306: F, t94635: F, t94641: F, t94648: F, t94650: F, t94662: F, t94665: F, t94672: F, t94675: F, t94677: F, t1955: F, t25949: F, t1883: F, t4131: F, t25912: F, t689: F, t3923: F, t4003: F, t1385: F, t7910: F, t14104: F, t94725: F, t1358: F, t785: F, t7925: F, t25904: F, t13920: F, t14224: F, t25933: F, t27864: F, t27980: F, t27981: F, t4056: F, t49380: F, t543: F, t7301: F, t94682: F, t94694: F, t94716: F, t212: F, t27960: F, t27899: F, t27873: F, t94890: F, t7929: F, t25944: F, t2470: F, t27887: F, t7284: F, t27836: F, t4075: F, t25934: F, t27865: F, t27869: F, t94700: F, t94703: F, t94705: F, t94714: F, t94726: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t97792, t97795, t97798, t97800, t97802) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2090::<F>(t2435, t27986, t1904, t2439, t25916, t26050, t27884, t25304, t27883, t25946, t25898, t97699);
        let (t97804, t97808, t97810, t97814, t97815) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2091::<F>(t25901, t97802, t1364, t27961, t786, t2453, t3908, t7911, t136, t2457, t7920, t94589);
        let t97827 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2092::<F>(t2435, t27965, t14090, t26054, t14268, t2022, t7295, t7296, t7921, t94608, t94610, t94613, t94616, t97792, t97795, t97798, t97800, t97804, t97808, t97810, t97815);
        let t97854 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2093::<F>(t25894, t97676, t97680, t1444, t5659, t14110, t94901, t10073, t1903, t2029, t25929, t25930, t25931, t27868, t49306, t94635, t94641, t94648, t94650, t94662, t94665, t94672, t94675, t94677);
        let (t97855, t97858, t97869, t97870, t97871, t97875) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2094::<F>(t1955, t25949, t1883, t4131, t1904, t25912, t689, t1903, t3923, t4003, t1385, t7910);
        let (t97899, t97903) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2095::<F>(t14104, t94725, t1358, t2439, t785, t7910, t2435, t7925, t25904, t13920, t14224, t2022, t25930, t25931, t25933, t27864, t27868, t27980, t27981, t4056, t49380, t543, t7295, t7301, t94682, t94694, t94716, t97855, t97858, t97869, t97871, t97875);
        let (t97908, t97909, t97915, t97917, t97920) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2096::<F>(t1358, t212, t27960, t689, t3923, t7910, t26050, t27899, t2453, t27883, t25946, t27873, t94890);
        let (t97922, t97925, t97938) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2097::<F>(t136, t2457, t7929, t25944, t2470, t27887, t7284, t1955, t27836, t4075, t25934, t27865, t27869, t543, t7295, t7301, t94700, t94703, t94705, t94714, t94726, t97855, t97908, t97909, t97915, t97917, t97920);
    (t97814, t97827, t97854, t97870, t97899, t97903, t97909, t97922, t97925, t97938)
}
