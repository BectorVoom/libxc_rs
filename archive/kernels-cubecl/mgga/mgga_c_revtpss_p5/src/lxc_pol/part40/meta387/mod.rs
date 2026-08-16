//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta387 (260520-c91 hierarchical CSE).
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
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1390;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1391;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1392;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1393;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1394;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1395;
use chunk6::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1396;
use chunk7::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1397;
use chunk8::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1398;
use chunk9::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1399;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta387<F: Float>(t1139: F, t16926: F, t16710: F, t5095: F, t698: F, t1132: F, t16708: F, t16717: F, t16722: F, t16735: F, t16740: F, t16744: F, t16908: F, t12252: F, t12261: F, t12263: F, t12265: F, t12349: F, t12352: F, t16731: F, t16852: F, t16855: F, t16858: F, t16860: F, t16863: F, t16865: F, t16883: F, t16887: F, t16890: F, t16893: F, t16895: F, t16898: F, t16901: F, t16904: F, t1150: F, t1131: F, t1168: F, t5143: F, t1745: F, t3471: F, t12423: F, t16649: F, t16651: F, t16654: F, t16657: F, t16660: F, t16664: F, t16667: F, t16671: F, t16690: F, t3452: F, t5147: F, t3453: F, t5146: F, t3479: F, t5142: F, t12472: F, t1744: F, t1757: F, t3497: F, t1187: F, t5181: F, t3515: F, t5184: F, t3523: F, t5180: F, t12429: F, t12470: F, t12481: F, t12486: F, t12491: F, t3477: F, t3496: F, t3521: F, t5163: F, t5185: F, t12555: F, t1756: F, t16712: F, t12297: F, t12299: F, t12301: F, t12303: F, t12397: F, t16706: F, t16727: F, t16748: F, t1737: F, t3451: F, t1160: F, t5117: F, t1170: F, t12511: F, t12553: F, t16809: F, t16832: F, t16837: F, t16839: F, t16842: F, t16844: F, t16846: F, t3454: F, t435: F, t5125: F, t3476: F, t16868: F, t16871: F, t16876: F, t16892: F, t12459: F, t12460: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t16927, t16931, t16933, t16940) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1390::<F>(t1139, t16926, t16710, t5095, t698, t1132, t16708, t16717, t16722, t16735, t16740, t16744, t16908);
        let t16942 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1391::<F>(t12252, t12261, t12263, t12265, t12349, t12352, t16731, t16852, t16855, t16858, t16860, t16863, t16865, t16883, t16887, t16890, t16893, t16895, t16898, t16901, t16904, t16940);
        let (t16945, t16954) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1392::<F>(t1150, t16942, t1131, t1168, t5143, t1745, t3471, t12423, t16649, t16651, t16654, t16657, t16660, t16664, t16667, t16671, t16690, t3452, t5147);
        let (t16955, t16959, t16962, t16966, t16971, t16974, t16979) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1393::<F>(t3453, t5146, t3479, t5142, t1168, t3471, t12472, t1744, t1757, t3497, t1745, t1187, t5181);
        let t16995 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1394::<F>(t1757, t3515, t3497, t5184, t3523, t5180, t1187, t12429, t12470, t12481, t12486, t12491, t16955, t16959, t16962, t16966, t16971, t16974, t16979, t3477, t3496, t3521, t5163, t5185);
        let (t16998, t17020) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1395::<F>(t12555, t1756, t3497, t16710, t16712, t12297, t12299, t12301, t12303, t12397, t16706, t16708, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
        let t17029 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1396::<F>(t1737, t3451, t1160, t5117, t1170, t12511, t12553, t16809, t16832, t16837, t16839, t16842, t16844, t16846, t16945, t16998, t17020, t3454, t435, t5125);
        let (t17032, t17061) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1397::<F>(t1737, t3476, t16868, t16712, t12297, t12299, t12301, t12303, t16706, t16727, t16748, t16871, t16876);
        let (t17066, t17083) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1398::<F>(t16892, t16708, t16710, t16717, t16722, t16735, t16740, t16744, t16908, t16927, t16931, t16933);
        let t17085 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1399::<F>(t12252, t12261, t12263, t12265, t12459, t12460, t16731, t16852, t16855, t16858, t16860, t16863, t16865, t16887, t16890, t16895, t16898, t16901, t16904, t17061, t17066, t17083);
    (t16927, t16931, t16933, t16945, t16954, t16995, t17029, t17032, t17085)
}
