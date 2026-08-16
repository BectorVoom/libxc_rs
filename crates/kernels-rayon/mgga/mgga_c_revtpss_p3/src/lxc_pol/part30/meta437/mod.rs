//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta437 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1677;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1678;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1679;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1680;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1681;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1682;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1683;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1684;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta437(t1169: f64, t17085: f64, t1179: f64, t5155: f64, t1719: f64, t3383: f64, t3386: f64, t1749: f64, t3520: f64, t16868: f64, t16712: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t16706: f64, t16727: f64, t16748: f64, t16871: f64, t16876: f64, t16892: f64, t16708: f64, t16710: f64, t16717: f64, t16722: f64, t16735: f64, t16740: f64, t16744: f64, t16908: f64, t16927: f64, t16931: f64, t16933: f64, t12252: f64, t12261: f64, t12263: f64, t12265: f64, t12542: f64, t12543: f64, t16731: f64, t16852: f64, t16855: f64, t16858: f64, t16860: f64, t16863: f64, t16865: f64, t16887: f64, t16890: f64, t16895: f64, t16898: f64, t16901: f64, t16904: f64, t1188: f64, t3495: f64, t1161: f64, t1180: f64, t1189: f64, t12418: f64, t12476: f64, t17032: f64, t1745: f64, t1757: f64, t3447: f64, t3472: f64, t3480: f64, t3491: f64, t3498: f64, t3516: f64, t3524: f64, t5120: f64, t5143: f64, t5158: f64, t5181: f64, t16954: f64, t16995: f64, t17029: f64, t300: f64, t3535: f64, t5192: f64, t1196: f64, t3531: f64, t5207: f64, t16783: f64, t16786: f64, t16788: f64, t16790: f64, t16809: f64, t16814: f64, t16834: f64, t16837: f64, t16839: f64, t16842: f64, t16844: f64, t16846: f64, t16945: f64, t16781: f64, t1287: f64, t487: f64, t3584: f64, t5486: f64, t16756: f64, t5480: f64, t1770: f64, t3781: f64, t1234: f64, t12709: f64, t12756: f64, t1285: f64, t1291: f64, t16697: f64, t16751: f64, t16757: f64, t16763: f64, t16768: f64, t16772: f64, t16776: f64, t3666: f64, t3670: f64, t3746: f64, t3760: f64, t3763: f64, t3784: f64, t5216: f64, t5326: f64, t5459: f64, t5463: f64, t5474: f64, t5478: f64, t5487: f64, t1248: f64, t5230: f64, t1284: f64, t1811: f64, t1209: f64, t13392: f64, t5268: f64, t1042: f64, t1263: f64, t3362: f64, t15936: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17086, t17089, t17094, t17097, t17126) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1677(t1169, t17085, t1179, t5155, t1719, t3383, t3386, t1749, t3520, t16868, t16712, t12297, t12299, t12301, t12303, t16706, t16727, t16748, t16871, t16876);
        let (t17131, t17148) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1678(t16892, t16708, t16710, t16717, t16722, t16735, t16740, t16744, t16908, t16927, t16931, t16933);
        let t17150 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1679(t12252, t12261, t12263, t12265, t12542, t12543, t16731, t16852, t16855, t16858, t16860, t16863, t16865, t16887, t16890, t16895, t16898, t16901, t16904, t17126, t17131, t17148);
        let t17157 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1680(t1188, t17150, t1749, t3495, t1161, t1180, t1189, t12418, t12476, t17032, t17086, t17089, t17094, t17097, t1745, t1757, t3447, t3472, t3480, t3491, t3498, t3516, t3524, t5120, t5143, t5158, t5181);
        let (t17160, t17162, t17166, t17168) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1681(t16954, t16995, t17029, t17157, t300, t3535, t5192, t1179, t1188, t17150, t1196, t3531, t5207);
        let t17169 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1682(t16783, t16786, t16788, t16790, t16809, t16814, t16834, t16837, t16839, t16842, t16844, t16846, t16945, t17094, t17160, t17162, t17166, t17168);
        let (t17170, t17186) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1683(t16781, t17169, t1287, t487, t3584, t5486, t16756, t5480, t1770, t3781, t1234, t12709, t12756, t1285, t1291, t16697, t16751, t16757, t16763, t16768, t16772, t16776, t3666, t3670, t3746, t3760, t3763, t3784, t5216, t5326, t5459, t5463, t5474, t5478, t5487);
        let (t17188, t17192, t17199, t17203) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1684(t1248, t1287, t5230, t1284, t1811, t1209, t13392, t5268, t1042, t1263, t3362, t15936);
    (t17094, t17160, t17162, t17166, t17168, t17170, t17186, t17188, t17192, t17199, t17203)
}
