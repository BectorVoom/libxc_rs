//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1054 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3725;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3726;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3727;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3728;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3729;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1054(t17708: f64, t59948: f64, t17394: f64, t370: f64, t17727: f64, t12916: f64, t21258: f64, t3718: f64, t17753: f64, t21045: f64, t12866: f64, t5401: f64, t58895: f64, t1250: f64, t12787: f64, t16733: f64, t17353: f64, t17420: f64, t17625: f64, t17693: f64, t17705: f64, t17713: f64, t17729: f64, t17730: f64, t17760: f64, t20265: f64, t20292: f64, t21014: f64, t44225: f64, t57421: f64, t57428: f64, t59411: f64, t20837: f64, t5331: f64, t12910: f64, t21003: f64, t5245: f64, t5284: f64, t1214: f64, t20836: f64, t12809: f64, t17605: f64, t17623: f64, t17669: f64, t17674: f64, t17677: f64, t17682: f64, t17703: f64, t17747: f64, t20956: f64, t21040: f64, t3720: f64, t5332: f64, t5340: f64, t57435: f64, t57449: f64, t57451: f64, t6421: f64, t1248: f64, t20950: f64, t21029: f64, t5333: f64, t12784: f64, t13396: f64, t17710: f64, t20795: f64, t20921: f64, t21157: f64, t44191: f64, t44548: f64, t57463: f64, t57471: f64, t57478: f64, t57486: f64, t57490: f64, t57508: f64, t21177: f64, t3678: f64, t17303: f64, t5327: f64, t13099: f64, t16715: f64, t16738: f64, t16742: f64, t17212: f64, t17426: f64, t17732: f64, t17737: f64, t17742: f64, t17781: f64, t17784: f64, t1794: f64, t20800: f64, t20802: f64, t20929: f64, t21017: f64, t3626: f64, t372: f64, t44561: f64, t57265: f64, t57534: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t70639, t70646, t70647, t70664, t70667, t70672) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3725(t17708, t59948, t17394, t370, t17727, t12916, t21258, t3718, t17753, t21045, t12866, t5401, t58895);
        let t70675 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3726(t1250, t12787, t16733, t17353, t17420, t17625, t17693, t17705, t17713, t17729, t17730, t17760, t20265, t20292, t21014, t44225, t57421, t57428, t59411, t70639, t70647, t70664, t70667, t70672);
        let (t70693, t70712, t70717) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3727(t12916, t20837, t5331, t12910, t21003, t5245, t5284, t1214, t20836, t1250, t12787, t12809, t17605, t17623, t17669, t17674, t17677, t17682, t17703, t17747, t20956, t21040, t3718, t3720, t5332, t5340, t57435, t57449, t57451, t6421);
        let (t70718, t70741, t70748) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3728(t1248, t20950, t12809, t12916, t21029, t5284, t5333, t12784, t12787, t13396, t17710, t17729, t17747, t17753, t20795, t20921, t21157, t3720, t44191, t44548, t5340, t57463, t57471, t57478, t57486, t57490, t57508);
        let t70789 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3729(t21177, t3678, t17303, t5327, t1250, t12809, t13099, t16715, t16738, t16742, t17212, t17353, t17426, t17693, t17732, t17737, t17742, t17781, t17784, t1794, t20795, t20800, t20802, t20929, t21017, t3626, t372, t3720, t44561, t5331, t57265, t57534, t70647);
    (t70646, t70675, t70693, t70712, t70717, t70718, t70741, t70748, t70789)
}
