//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta445 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1679;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1680;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1681;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1682;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1683;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1684;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1685;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1686;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta445<F: Float>(t1169: F, t17085: F, t1179: F, t5155: F, t1719: F, t3383: F, t3386: F, t1749: F, t3520: F, t16868: F, t16712: F, t12297: F, t12299: F, t12301: F, t12303: F, t16706: F, t16727: F, t16748: F, t16871: F, t16876: F, t16892: F, t16708: F, t16710: F, t16717: F, t16722: F, t16735: F, t16740: F, t16744: F, t16908: F, t16927: F, t16931: F, t16933: F, t12252: F, t12261: F, t12263: F, t12265: F, t12542: F, t12543: F, t16731: F, t16852: F, t16855: F, t16858: F, t16860: F, t16863: F, t16865: F, t16887: F, t16890: F, t16895: F, t16898: F, t16901: F, t16904: F, t1188: F, t3495: F, t1161: F, t1180: F, t1189: F, t12418: F, t12476: F, t17032: F, t1745: F, t1757: F, t3447: F, t3472: F, t3480: F, t3491: F, t3498: F, t3516: F, t3524: F, t5120: F, t5143: F, t5158: F, t5181: F, t16954: F, t16995: F, t17029: F, t300: F, t3535: F, t5192: F, t1196: F, t3531: F, t5207: F, t16783: F, t16786: F, t16788: F, t16790: F, t16809: F, t16814: F, t16834: F, t16837: F, t16839: F, t16842: F, t16844: F, t16846: F, t16945: F, t16781: F, t1287: F, t487: F, t3584: F, t5486: F, t16756: F, t5480: F, t1770: F, t3781: F, t1234: F, t12709: F, t12756: F, t1285: F, t1291: F, t16697: F, t16751: F, t16757: F, t16763: F, t16768: F, t16772: F, t16776: F, t3666: F, t3670: F, t3746: F, t3760: F, t3763: F, t3784: F, t5216: F, t5326: F, t5459: F, t5463: F, t5474: F, t5478: F, t5487: F, t1248: F, t5230: F, t1284: F, t1811: F, t1209: F, t13392: F, t5268: F, t1042: F, t1263: F, t3362: F, t15936: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17086, t17089, t17094, t17097, t17126) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1679::<F>(t1169, t17085, t1179, t5155, t1719, t3383, t3386, t1749, t3520, t16868, t16712, t12297, t12299, t12301, t12303, t16706, t16727, t16748, t16871, t16876);
        let (t17131, t17148) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1680::<F>(t16892, t16708, t16710, t16717, t16722, t16735, t16740, t16744, t16908, t16927, t16931, t16933);
        let t17150 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1681::<F>(t12252, t12261, t12263, t12265, t12542, t12543, t16731, t16852, t16855, t16858, t16860, t16863, t16865, t16887, t16890, t16895, t16898, t16901, t16904, t17126, t17131, t17148);
        let t17157 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1682::<F>(t1188, t17150, t1749, t3495, t1161, t1180, t1189, t12418, t12476, t17032, t17086, t17089, t17094, t17097, t1745, t1757, t3447, t3472, t3480, t3491, t3498, t3516, t3524, t5120, t5143, t5158, t5181);
        let (t17160, t17162, t17166, t17168) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1683::<F>(t16954, t16995, t17029, t17157, t300, t3535, t5192, t1179, t1188, t17150, t1196, t3531, t5207);
        let t17169 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1684::<F>(t16783, t16786, t16788, t16790, t16809, t16814, t16834, t16837, t16839, t16842, t16844, t16846, t16945, t17094, t17160, t17162, t17166, t17168);
        let (t17170, t17186) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1685::<F>(t16781, t17169, t1287, t487, t3584, t5486, t16756, t5480, t1770, t3781, t1234, t12709, t12756, t1285, t1291, t16697, t16751, t16757, t16763, t16768, t16772, t16776, t3666, t3670, t3746, t3760, t3763, t3784, t5216, t5326, t5459, t5463, t5474, t5478, t5487);
        let (t17188, t17192, t17199, t17203) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1686::<F>(t1248, t1287, t5230, t1284, t1811, t1209, t13392, t5268, t1042, t1263, t3362, t15936);
    (t17094, t17160, t17162, t17166, t17168, t17170, t17186, t17188, t17192, t17199, t17203)
}
