//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta545 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2210;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2211;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2212;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2213;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2214;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2215;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2216;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta545(t1737: f64, t3476: f64, t16868: f64, t16712: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t16706: f64, t16727: f64, t16748: f64, t16871: f64, t16876: f64, t16892: f64, t16708: f64, t16710: f64, t16717: f64, t16722: f64, t16735: f64, t16740: f64, t16744: f64, t16908: f64, t16927: f64, t16931: f64, t16933: f64, t12252: f64, t12261: f64, t12263: f64, t12265: f64, t12459: f64, t12460: f64, t16731: f64, t16852: f64, t16855: f64, t16858: f64, t16860: f64, t16863: f64, t16865: f64, t16887: f64, t16890: f64, t16895: f64, t16898: f64, t16901: f64, t16904: f64, t1169: f64, t1179: f64, t5155: f64, t1719: f64, t3383: f64, t3386: f64, t1749: f64, t3520: f64, t12542: f64, t12543: f64, t1188: f64, t3495: f64, t1161: f64, t1180: f64, t1189: f64, t12418: f64, t12476: f64, t1745: f64, t1757: f64, t3447: f64, t3472: f64, t3480: f64, t3491: f64, t3498: f64, t3516: f64, t3524: f64, t5120: f64, t5143: f64, t5158: f64, t5181: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17032, t17061) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2210(t1737, t3476, t16868, t16712, t12297, t12299, t12301, t12303, t16706, t16727, t16748, t16871, t16876);
        let (t17066, t17083) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2211(t16892, t16708, t16710, t16717, t16722, t16735, t16740, t16744, t16908, t16927, t16931, t16933);
        let t17085 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2212(t12252, t12261, t12263, t12265, t12459, t12460, t16731, t16852, t16855, t16858, t16860, t16863, t16865, t16887, t16890, t16895, t16898, t16901, t16904, t17061, t17066, t17083);
        let (t17086, t17089, t17092, t17094, t17097, t17126) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2213(t1169, t17085, t1179, t5155, t1719, t3383, t3386, t1749, t3520, t16868, t16712, t12297, t12299, t12301, t12303, t16706, t16727, t16748, t16871, t16876);
        let (t17131, t17148) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2214(t16892, t16708, t16710, t16717, t16722, t16735, t16740, t16744, t16908, t16927, t16931, t16933);
        let t17150 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2215(t12252, t12261, t12263, t12265, t12542, t12543, t16731, t16852, t16855, t16858, t16860, t16863, t16865, t16887, t16890, t16895, t16898, t16901, t16904, t17126, t17131, t17148);
        let (t17151, t17154, t17157) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2216(t1188, t17150, t1749, t3495, t1161, t1180, t1189, t12418, t12476, t17032, t17086, t17089, t17094, t17097, t1745, t1757, t3447, t3472, t3480, t3491, t3498, t3516, t3524, t5120, t5143, t5158, t5181);
    (t17032, t17085, t17086, t17089, t17092, t17094, t17097, t17150, t17151, t17154, t17157)
}
