//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta379 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1355;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1356;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1357;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1358;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1359;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1360;
use chunk6::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1361;
use chunk7::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1362;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta379<F: Float>(t16551: F, t342: F, t11631: F, t12050: F, t3151: F, t15907: F, t12077: F, t378: F, t3154: F, t12046: F, t357: F, t3133: F, t3302: F, t4893: F, t3059: F, t4975: F, t4781: F, t12132: F, t1647: F, t3316: F, t1083: F, t12122: F, t12127: F, t12146: F, t12149: F, t12154: F, t15655: F, t16529: F, t16534: F, t16537: F, t16540: F, t16544: F, t3278: F, t3288: F, t3309: F, t3319: F, t4954: F, t4964: F, t4977: F, t4981: F, t4996: F, t5009: F, t16423: F, t16475: F, t16526: F, t1079: F, t1071: F, t4746: F, t15669: F, t379: F, t994: F, t1695: F, t3268: F, t3066: F, t1000: F, t1076: F, t1097: F, t11128: F, t11210: F, t11214: F, t16362: F, t16371: F, t16374: F, t1652: F, t1696: F, t3047: F, t3060: F, t3067: F, t3076: F, t3264: F, t4747: F, t4773: F, t4778: F, t5016: F, t16272: F, t16310: F, t16355: F, t1100: F, t1102: F, t15418: F, t15420: F, t15423: F, t15425: F, t15427: F, t15477: F, t15515: F, t15549: F, t15551: F, t15553: F, t15555: F, t15558: F, t15561: F, t15562: F, t15566: F, t15571: F, t15575: F, t15577: F, t16181: F, t198: F, t3333: F, t336: F, t5023: F, t30: F, t265: F, t393: F, t15083: F, t15546: F, t1106: F, t13312: F, t1468: F, t1469: F, t15093: F, t15094: F, t15096: F, t1587: F, t1704: F, t2257: F, t2258: F, t2838: F, t3340: F, t395: F, t4186: F, t45: F, t4560: F, t5028: F, t605: F, t606: F, dens_threshold: F, rho0: F, zeta_threshold: F, t3498: F, t5205: F, t1196: F, t12485: F, t1756: F, t3524: F, t3531: F, t5198: F, t12361: F, t5068: F, t12243: F, t5109: F, t1149: F, t5105: F, t3384: F, t1733: F, t3427: F, t3385: F, t5108: F, t12248: F, t3435: F, t5104: F, t3433: F, t12230: F, t1732: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16552, t16555, t16559, t16562, t16566, t16569, t16573) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1355::<F>(t16551, t342, t11631, t12050, t3151, t15907, t12077, t378, t3154, t12046, t357, t3133, t3302);
        let t16589 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1356::<F>(t16573, t4893, t3059, t4975, t4781, t12132, t1647, t3316, t1083, t12122, t12127, t12146, t12149, t12154, t15655, t16529, t16534, t16537, t16540, t16544, t16552, t16555, t16559, t16562, t16566, t16569, t3278, t3288, t3309, t3319, t342, t4954, t4964, t4977, t4981, t4996, t5009);
        let (t16592, t16597, t16600, t16603, t16604) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1357::<F>(t16423, t16475, t16526, t16589, t1079, t1071, t4746, t15669, t378, t379, t994, t1695, t3268);
        let t16610 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1358::<F>(t16604, t3066, t1000, t1076, t1097, t11128, t11210, t11214, t16362, t16371, t16374, t1652, t16592, t16597, t16600, t16603, t1696, t3047, t3060, t3067, t3076, t3264, t4747, t4773, t4778, t5016);
        let t16616 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1359::<F>(t16272, t16310, t16355, t16610, t1100, t1102, t15418, t15420, t15423, t15425, t15427, t15477, t15515, t15549, t15551, t15553, t15555, t15558, t15561, t15562, t15566, t15571, t15575, t15577, t16181, t198, t3333, t336, t5023);
        let t16630 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1360::<F>(t30, t265, t393, t15083, t15546, t16616, t1106, t13312, t1468, t1469, t15093, t15094, t15096, t1587, t1704, t2257, t2258, t2838, t3340, t395, t4186, t45, t4560, t5028, t605, t606, dens_threshold, rho0, zeta_threshold);
        let (t16641, t16645, t16647, t16649, t16651) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1361::<F>(t3498, t5205, t1196, t12485, t1756, t3524, t3531, t5198, t12361, t5068, t12243, t5109);
        let (t16654, t16657, t16660, t16664, t16667, t16668) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1362::<F>(t1149, t5105, t3384, t1733, t3427, t3385, t5108, t12248, t3435, t5104, t3433, t12230, t1732);
    (t16630, t16641, t16645, t16647, t16649, t16651, t16654, t16657, t16660, t16664, t16667, t16668)
}
