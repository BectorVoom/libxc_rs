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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta609(t25875: f64, t97703: f64, t97705: f64, t122: f64, t3916: f64, t72: f64, t7910: f64, t25895: f64, t1398: f64, t543: f64, t5774: f64, t1903: f64, t4056: f64, t1444: f64, t25921: f64, t25930: f64, t25931: f64, t27903: f64, t27960: f64, t28003: f64, t7274: f64, t7295: f64, t7296: f64, t94405: f64, t94409: f64, t94411: f64, t94580: f64, t94584: f64, t94591: f64, t2022: f64, t9990: f64, t1426: f64, t786: f64, t7911: f64, t3917: f64, t14230: f64, t25924: f64, t25926: f64, t27837: f64, t27868: f64, t27973: f64, t27980: f64, t3999: f64, t4077: f64, t4131: f64, t48020: f64, t48074: f64, t49393: f64, t7920: f64, t94593: f64, t94598: f64, t94602: f64, t94605: f64, t94656: f64, t94705: f64, t2435: f64, t27986: f64, t1904: f64, t2439: f64, t25916: f64, t26050: f64, t27884: f64, t25304: f64, t27883: f64, t25946: f64, t25898: f64, t97699: f64, t25901: f64, t1364: f64, t27961: f64, t2453: f64, t3908: f64, t136: f64, t2457: f64, t94589: f64, t27965: f64, t14090: f64, t26054: f64, t14268: f64, t7921: f64, t94608: f64, t94610: f64, t94613: f64, t94616: f64, t25894: f64, t97676: f64, t97680: f64, t5659: f64, t14110: f64, t94901: f64, t10073: f64, t2029: f64, t25929: f64, t49306: f64, t94635: f64, t94641: f64, t94648: f64, t94650: f64, t94662: f64, t94665: f64, t94672: f64, t94675: f64, t94677: f64, t1955: f64, t25949: f64, t1883: f64, t25912: f64, t689: f64, t3923: f64, t4003: f64, t1385: f64, t14104: f64, t94725: f64, t1358: f64, t785: f64, t7925: f64, t25904: f64, t13920: f64, t14224: f64, t25933: f64, t27864: f64, t27981: f64, t49380: f64, t7301: f64, t94682: f64, t94694: f64, t94716: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97719, t97732, t97734, t97737, t97742) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2075(t25875, t97703, t97705, t122, t3916, t72, t7910, t25895, t1398, t543, t5774, t1903, t4056);
        let t97752 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2076(t1444, t25921, t25930, t25931, t27903, t27960, t28003, t5774, t7274, t7295, t7296, t94405, t94409, t94411, t94580, t94584, t94591, t97719, t97734, t97737, t97742);
        let t97791 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2077(t2022, t9990, t1426, t786, t7911, t3917, t14230, t25924, t25926, t27837, t27868, t27973, t27980, t3999, t4077, t4131, t48020, t48074, t49393, t7274, t7295, t7296, t7910, t7920, t94593, t94598, t94602, t94605, t94656, t94705);
        let (t97792, t97795, t97798, t97800, t97802) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2078(t2435, t27986, t1904, t2439, t25916, t26050, t27884, t25304, t27883, t25946, t25898, t97699);
        let (t97804, t97808, t97810, t97814, t97815) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2079(t25901, t97802, t1364, t27961, t786, t2453, t3908, t7911, t136, t2457, t7920, t94589);
        let t97827 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2080(t2435, t27965, t14090, t26054, t14268, t2022, t7295, t7296, t7921, t94608, t94610, t94613, t94616, t97792, t97795, t97798, t97800, t97804, t97808, t97810, t97815);
        let t97854 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2081(t25894, t97676, t97680, t1444, t5659, t14110, t94901, t10073, t1903, t2029, t25929, t25930, t25931, t27868, t49306, t94635, t94641, t94648, t94650, t94662, t94665, t94672, t94675, t94677);
        let (t97855, t97858, t97869, t97870, t97871, t97875) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2082(t1955, t25949, t1883, t4131, t1904, t25912, t689, t1903, t3923, t4003, t1385, t7910);
        let (t97899, t97903) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2083(t14104, t94725, t1358, t2439, t785, t7910, t2435, t7925, t25904, t13920, t14224, t2022, t25930, t25931, t25933, t27864, t27868, t27980, t27981, t4056, t49380, t543, t7295, t7301, t94682, t94694, t94716, t97855, t97858, t97869, t97871, t97875);
    (t97732, t97752, t97791, t97814, t97827, t97854, t97855, t97870, t97899, t97903)
}
