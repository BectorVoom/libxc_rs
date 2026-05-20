//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta545 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2210;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2211;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2212;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2213;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2214;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2215;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2216;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta545<F: Float>(t1737: F, t3476: F, t16868: F, t16712: F, t12297: F, t12299: F, t12301: F, t12303: F, t16706: F, t16727: F, t16748: F, t16871: F, t16876: F, t16892: F, t16708: F, t16710: F, t16717: F, t16722: F, t16735: F, t16740: F, t16744: F, t16908: F, t16927: F, t16931: F, t16933: F, t12252: F, t12261: F, t12263: F, t12265: F, t12459: F, t12460: F, t16731: F, t16852: F, t16855: F, t16858: F, t16860: F, t16863: F, t16865: F, t16887: F, t16890: F, t16895: F, t16898: F, t16901: F, t16904: F, t1169: F, t1179: F, t5155: F, t1719: F, t3383: F, t3386: F, t1749: F, t3520: F, t12542: F, t12543: F, t1188: F, t3495: F, t1161: F, t1180: F, t1189: F, t12418: F, t12476: F, t1745: F, t1757: F, t3447: F, t3472: F, t3480: F, t3491: F, t3498: F, t3516: F, t3524: F, t5120: F, t5143: F, t5158: F, t5181: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17032, t17061) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2210::<F>(t1737, t3476, t16868, t16712, t12297, t12299, t12301, t12303, t16706, t16727, t16748, t16871, t16876);
        let (t17066, t17083) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2211::<F>(t16892, t16708, t16710, t16717, t16722, t16735, t16740, t16744, t16908, t16927, t16931, t16933);
        let t17085 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2212::<F>(t12252, t12261, t12263, t12265, t12459, t12460, t16731, t16852, t16855, t16858, t16860, t16863, t16865, t16887, t16890, t16895, t16898, t16901, t16904, t17061, t17066, t17083);
        let (t17086, t17089, t17092, t17094, t17097, t17126) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2213::<F>(t1169, t17085, t1179, t5155, t1719, t3383, t3386, t1749, t3520, t16868, t16712, t12297, t12299, t12301, t12303, t16706, t16727, t16748, t16871, t16876);
        let (t17131, t17148) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2214::<F>(t16892, t16708, t16710, t16717, t16722, t16735, t16740, t16744, t16908, t16927, t16931, t16933);
        let t17150 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2215::<F>(t12252, t12261, t12263, t12265, t12542, t12543, t16731, t16852, t16855, t16858, t16860, t16863, t16865, t16887, t16890, t16895, t16898, t16901, t16904, t17126, t17131, t17148);
        let (t17151, t17154, t17157) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2216::<F>(t1188, t17150, t1749, t3495, t1161, t1180, t1189, t12418, t12476, t17032, t17086, t17089, t17094, t17097, t1745, t1757, t3447, t3472, t3480, t3491, t3498, t3516, t3524, t5120, t5143, t5158, t5181);
    (t17032, t17085, t17086, t17089, t17092, t17094, t17097, t17150, t17151, t17154, t17157)
}
