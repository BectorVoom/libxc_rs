//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta609 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2075;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2076;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2077;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2078;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2079;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2080;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2081;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2082;
use chunk8::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2083;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta609<F: Float>(t25875: F, t97703: F, t97705: F, t122: F, t3916: F, t72: F, t7910: F, t25895: F, t1398: F, t543: F, t5774: F, t1903: F, t4056: F, t1444: F, t25921: F, t25930: F, t25931: F, t27903: F, t27960: F, t28003: F, t7274: F, t7295: F, t7296: F, t94405: F, t94409: F, t94411: F, t94580: F, t94584: F, t94591: F, t2022: F, t9990: F, t1426: F, t786: F, t7911: F, t3917: F, t14230: F, t25924: F, t25926: F, t27837: F, t27868: F, t27973: F, t27980: F, t3999: F, t4077: F, t4131: F, t48020: F, t48074: F, t49393: F, t7920: F, t94593: F, t94598: F, t94602: F, t94605: F, t94656: F, t94705: F, t2435: F, t27986: F, t1904: F, t2439: F, t25916: F, t26050: F, t27884: F, t25304: F, t27883: F, t25946: F, t25898: F, t97699: F, t25901: F, t1364: F, t27961: F, t2453: F, t3908: F, t136: F, t2457: F, t94589: F, t27965: F, t14090: F, t26054: F, t14268: F, t7921: F, t94608: F, t94610: F, t94613: F, t94616: F, t25894: F, t97676: F, t97680: F, t5659: F, t14110: F, t94901: F, t10073: F, t2029: F, t25929: F, t49306: F, t94635: F, t94641: F, t94648: F, t94650: F, t94662: F, t94665: F, t94672: F, t94675: F, t94677: F, t1955: F, t25949: F, t1883: F, t25912: F, t689: F, t3923: F, t4003: F, t1385: F, t14104: F, t94725: F, t1358: F, t785: F, t7925: F, t25904: F, t13920: F, t14224: F, t25933: F, t27864: F, t27981: F, t49380: F, t7301: F, t94682: F, t94694: F, t94716: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t97719, t97732, t97734, t97737, t97742) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2075::<F>(t25875, t97703, t97705, t122, t3916, t72, t7910, t25895, t1398, t543, t5774, t1903, t4056);
        let t97752 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2076::<F>(t1444, t25921, t25930, t25931, t27903, t27960, t28003, t5774, t7274, t7295, t7296, t94405, t94409, t94411, t94580, t94584, t94591, t97719, t97734, t97737, t97742);
        let t97791 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2077::<F>(t2022, t9990, t1426, t786, t7911, t3917, t14230, t25924, t25926, t27837, t27868, t27973, t27980, t3999, t4077, t4131, t48020, t48074, t49393, t7274, t7295, t7296, t7910, t7920, t94593, t94598, t94602, t94605, t94656, t94705);
        let (t97792, t97795, t97798, t97800, t97802) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2078::<F>(t2435, t27986, t1904, t2439, t25916, t26050, t27884, t25304, t27883, t25946, t25898, t97699);
        let (t97804, t97808, t97810, t97814, t97815) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2079::<F>(t25901, t97802, t1364, t27961, t786, t2453, t3908, t7911, t136, t2457, t7920, t94589);
        let t97827 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2080::<F>(t2435, t27965, t14090, t26054, t14268, t2022, t7295, t7296, t7921, t94608, t94610, t94613, t94616, t97792, t97795, t97798, t97800, t97804, t97808, t97810, t97815);
        let t97854 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2081::<F>(t25894, t97676, t97680, t1444, t5659, t14110, t94901, t10073, t1903, t2029, t25929, t25930, t25931, t27868, t49306, t94635, t94641, t94648, t94650, t94662, t94665, t94672, t94675, t94677);
        let (t97855, t97858, t97869, t97870, t97871, t97875) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2082::<F>(t1955, t25949, t1883, t4131, t1904, t25912, t689, t1903, t3923, t4003, t1385, t7910);
        let (t97899, t97903) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2083::<F>(t14104, t94725, t1358, t2439, t785, t7910, t2435, t7925, t25904, t13920, t14224, t2022, t25930, t25931, t25933, t27864, t27868, t27980, t27981, t4056, t49380, t543, t7295, t7301, t94682, t94694, t94716, t97855, t97858, t97869, t97871, t97875);
    (t97732, t97752, t97791, t97814, t97827, t97854, t97855, t97870, t97899, t97903)
}
