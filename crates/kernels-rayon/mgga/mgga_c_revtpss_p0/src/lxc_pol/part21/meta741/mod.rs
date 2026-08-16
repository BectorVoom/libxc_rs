//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta741 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2607;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2608;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2609;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2610;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2611;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2612;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta741(t10014: f64, t14216: f64, t13921: f64, t4101: f64, t686: f64, t72: f64, t10139: f64, t136: f64, t2457: f64, t5659: f64, t14202: f64, t9303: f64, t14238: f64, t2453: f64, t10142: f64, t10019: f64, t14239: f64, t1882: f64, t4066: f64, t1398: f64, t21990: f64, t10022: f64, t2782: f64, t46463: f64, t46465: f64, t5675: f64, t5745: f64, t5767: f64, t820: f64, t9891: f64, t13790: f64, t4056: f64, t10073: f64, t14231: f64, t14219: f64, t9285: f64, t14215: f64, t2470: f64, t14220: f64, t46495: f64, t4086: f64, t5710: f64, t786: f64, t4104: f64, t14255: f64, t1883: f64, t3924: f64, t46472: f64, t46490: f64, t46493: f64, t47396: f64, t1437: f64, t2482: f64, t5658: f64, t543: f64, t3923: f64, t4003: f64, t14242: f64, t14225: f64, t1892: f64, t5744: f64, t10026: f64, t3964: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47995, t47999, t48004, t48005) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2607(t10014, t14216, t13921, t4101, t686, t72, t10139, t136, t2457, t5659, t14202, t9303);
        let (t48015, t48024) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2608(t14238, t2453, t10142, t10019, t14239, t1882, t4066, t1398, t21990, t10022, t2782, t46463, t46465, t47995, t47999, t48004, t48005, t5675, t5745, t5767, t820, t9891);
        let (t48027, t48029, t48036, t48039) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2609(t13790, t4056, t10022, t2782, t10073, t14231, t10139, t14219, t9285, t14215, t2470, t4101);
        let t48052 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2610(t48039, t14220, t46495, t4086, t5710, t786, t4104, t14255, t1883, t3924, t46472, t46490, t46493, t47396, t48027, t48029, t48036, t820);
        let (t48058, t48066, t48073, t48076) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2611(t1437, t2482, t4104, t5658, t2782, t4086, t48015, t543, t1882, t3923, t4003, t10022);
        let (t48080, t48082, t48085, t48089) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2612(t10014, t14242, t10073, t14225, t1892, t5744, t786, t10026, t136, t2457, t3964, t5710);
    (t48015, t48024, t48052, t48058, t48066, t48073, t48076, t48080, t48082, t48085, t48089)
}
