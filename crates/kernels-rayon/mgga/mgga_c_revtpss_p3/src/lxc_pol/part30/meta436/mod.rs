//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta436 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1667;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1668;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1669;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1670;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1671;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1672;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1673;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1674;
use chunk8::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1675;
use chunk9::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1676;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta436(t1139: f64, t16926: f64, t16710: f64, t5095: f64, t698: f64, t1132: f64, t16708: f64, t16717: f64, t16722: f64, t16735: f64, t16740: f64, t16744: f64, t16908: f64, t12252: f64, t12261: f64, t12263: f64, t12265: f64, t12349: f64, t12352: f64, t16731: f64, t16852: f64, t16855: f64, t16858: f64, t16860: f64, t16863: f64, t16865: f64, t16883: f64, t16887: f64, t16890: f64, t16893: f64, t16895: f64, t16898: f64, t16901: f64, t16904: f64, t1150: f64, t1131: f64, t1168: f64, t5143: f64, t1745: f64, t3471: f64, t12423: f64, t16649: f64, t16651: f64, t16654: f64, t16657: f64, t16660: f64, t16664: f64, t16667: f64, t16671: f64, t16690: f64, t3452: f64, t5147: f64, t3453: f64, t5146: f64, t3479: f64, t5142: f64, t12472: f64, t1744: f64, t1757: f64, t3497: f64, t1187: f64, t5181: f64, t3515: f64, t5184: f64, t3523: f64, t5180: f64, t12429: f64, t12470: f64, t12481: f64, t12486: f64, t12491: f64, t3477: f64, t3496: f64, t3521: f64, t5163: f64, t5185: f64, t12555: f64, t1756: f64, t16712: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12397: f64, t16706: f64, t16727: f64, t16748: f64, t1737: f64, t3451: f64, t1160: f64, t5117: f64, t1170: f64, t12511: f64, t12553: f64, t16809: f64, t16832: f64, t16837: f64, t16839: f64, t16842: f64, t16844: f64, t16846: f64, t3454: f64, t435: f64, t5125: f64, t3476: f64, t16868: f64, t16871: f64, t16876: f64, t16892: f64, t12459: f64, t12460: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16927, t16931, t16933, t16940) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1667(t1139, t16926, t16710, t5095, t698, t1132, t16708, t16717, t16722, t16735, t16740, t16744, t16908);
        let t16942 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1668(t12252, t12261, t12263, t12265, t12349, t12352, t16731, t16852, t16855, t16858, t16860, t16863, t16865, t16883, t16887, t16890, t16893, t16895, t16898, t16901, t16904, t16940);
        let (t16945, t16954) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1669(t1150, t16942, t1131, t1168, t5143, t1745, t3471, t12423, t16649, t16651, t16654, t16657, t16660, t16664, t16667, t16671, t16690, t3452, t5147);
        let (t16955, t16959, t16962, t16966, t16971, t16974, t16979) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1670(t3453, t5146, t3479, t5142, t1168, t3471, t12472, t1744, t1757, t3497, t1745, t1187, t5181);
        let t16995 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1671(t1757, t3515, t3497, t5184, t3523, t5180, t1187, t12429, t12470, t12481, t12486, t12491, t16955, t16959, t16962, t16966, t16971, t16974, t16979, t3477, t3496, t3521, t5163, t5185);
        let (t16998, t17020) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1672(t12555, t1756, t3497, t16710, t16712, t12297, t12299, t12301, t12303, t12397, t16706, t16708, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
        let t17029 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1673(t1737, t3451, t1160, t5117, t1170, t12511, t12553, t16809, t16832, t16837, t16839, t16842, t16844, t16846, t16945, t16998, t17020, t3454, t435, t5125);
        let (t17032, t17061) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1674(t1737, t3476, t16868, t16712, t12297, t12299, t12301, t12303, t16706, t16727, t16748, t16871, t16876);
        let (t17066, t17083) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1675(t16892, t16708, t16710, t16717, t16722, t16735, t16740, t16744, t16908, t16927, t16931, t16933);
        let t17085 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1676(t12252, t12261, t12263, t12265, t12459, t12460, t16731, t16852, t16855, t16858, t16860, t16863, t16865, t16887, t16890, t16895, t16898, t16901, t16904, t17061, t17066, t17083);
    (t16927, t16931, t16933, t16945, t16954, t16995, t17029, t17032, t17085)
}
