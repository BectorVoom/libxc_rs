//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1054 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3725;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3726;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3727;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3728;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3729;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1054<F: Float>(t17708: F, t59948: F, t17394: F, t370: F, t17727: F, t12916: F, t21258: F, t3718: F, t17753: F, t21045: F, t12866: F, t5401: F, t58895: F, t1250: F, t12787: F, t16733: F, t17353: F, t17420: F, t17625: F, t17693: F, t17705: F, t17713: F, t17729: F, t17730: F, t17760: F, t20265: F, t20292: F, t21014: F, t44225: F, t57421: F, t57428: F, t59411: F, t20837: F, t5331: F, t12910: F, t21003: F, t5245: F, t5284: F, t1214: F, t20836: F, t12809: F, t17605: F, t17623: F, t17669: F, t17674: F, t17677: F, t17682: F, t17703: F, t17747: F, t20956: F, t21040: F, t3720: F, t5332: F, t5340: F, t57435: F, t57449: F, t57451: F, t6421: F, t1248: F, t20950: F, t21029: F, t5333: F, t12784: F, t13396: F, t17710: F, t20795: F, t20921: F, t21157: F, t44191: F, t44548: F, t57463: F, t57471: F, t57478: F, t57486: F, t57490: F, t57508: F, t21177: F, t3678: F, t17303: F, t5327: F, t13099: F, t16715: F, t16738: F, t16742: F, t17212: F, t17426: F, t17732: F, t17737: F, t17742: F, t17781: F, t17784: F, t1794: F, t20800: F, t20802: F, t20929: F, t21017: F, t3626: F, t372: F, t44561: F, t57265: F, t57534: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t70639, t70646, t70647, t70664, t70667, t70672) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3725::<F>(t17708, t59948, t17394, t370, t17727, t12916, t21258, t3718, t17753, t21045, t12866, t5401, t58895);
        let t70675 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3726::<F>(t1250, t12787, t16733, t17353, t17420, t17625, t17693, t17705, t17713, t17729, t17730, t17760, t20265, t20292, t21014, t44225, t57421, t57428, t59411, t70639, t70647, t70664, t70667, t70672);
        let (t70693, t70712, t70717) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3727::<F>(t12916, t20837, t5331, t12910, t21003, t5245, t5284, t1214, t20836, t1250, t12787, t12809, t17605, t17623, t17669, t17674, t17677, t17682, t17703, t17747, t20956, t21040, t3718, t3720, t5332, t5340, t57435, t57449, t57451, t6421);
        let (t70718, t70741, t70748) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3728::<F>(t1248, t20950, t12809, t12916, t21029, t5284, t5333, t12784, t12787, t13396, t17710, t17729, t17747, t17753, t20795, t20921, t21157, t3720, t44191, t44548, t5340, t57463, t57471, t57478, t57486, t57490, t57508);
        let t70789 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3729::<F>(t21177, t3678, t17303, t5327, t1250, t12809, t13099, t16715, t16738, t16742, t17212, t17353, t17426, t17693, t17732, t17737, t17742, t17781, t17784, t1794, t20795, t20800, t20802, t20929, t21017, t3626, t372, t3720, t44561, t5331, t57265, t57534, t70647);
    (t70646, t70675, t70693, t70712, t70717, t70718, t70741, t70748, t70789)
}
