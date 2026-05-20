//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta741 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2607;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2608;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2609;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2610;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2611;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2612;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta741<F: Float>(t10014: F, t14216: F, t13921: F, t4101: F, t686: F, t72: F, t10139: F, t136: F, t2457: F, t5659: F, t14202: F, t9303: F, t14238: F, t2453: F, t10142: F, t10019: F, t14239: F, t1882: F, t4066: F, t1398: F, t21990: F, t10022: F, t2782: F, t46463: F, t46465: F, t5675: F, t5745: F, t5767: F, t820: F, t9891: F, t13790: F, t4056: F, t10073: F, t14231: F, t14219: F, t9285: F, t14215: F, t2470: F, t14220: F, t46495: F, t4086: F, t5710: F, t786: F, t4104: F, t14255: F, t1883: F, t3924: F, t46472: F, t46490: F, t46493: F, t47396: F, t1437: F, t2482: F, t5658: F, t543: F, t3923: F, t4003: F, t14242: F, t14225: F, t1892: F, t5744: F, t10026: F, t3964: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t47995, t47999, t48004, t48005) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2607::<F>(t10014, t14216, t13921, t4101, t686, t72, t10139, t136, t2457, t5659, t14202, t9303);
        let (t48015, t48024) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2608::<F>(t14238, t2453, t10142, t10019, t14239, t1882, t4066, t1398, t21990, t10022, t2782, t46463, t46465, t47995, t47999, t48004, t48005, t5675, t5745, t5767, t820, t9891);
        let (t48027, t48029, t48036, t48039) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2609::<F>(t13790, t4056, t10022, t2782, t10073, t14231, t10139, t14219, t9285, t14215, t2470, t4101);
        let t48052 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2610::<F>(t48039, t14220, t46495, t4086, t5710, t786, t4104, t14255, t1883, t3924, t46472, t46490, t46493, t47396, t48027, t48029, t48036, t820);
        let (t48058, t48066, t48073, t48076) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2611::<F>(t1437, t2482, t4104, t5658, t2782, t4086, t48015, t543, t1882, t3923, t4003, t10022);
        let (t48080, t48082, t48085, t48089) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2612::<F>(t10014, t14242, t10073, t14225, t1892, t5744, t786, t10026, t136, t2457, t3964, t5710);
    (t48015, t48024, t48052, t48058, t48066, t48073, t48076, t48080, t48082, t48085, t48089)
}
