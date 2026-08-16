//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1637;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1638;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1639;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1640;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1641;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1642;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta435(t16551: f64, t342: f64, t11631: f64, t12050: f64, t3151: f64, t15907: f64, t12077: f64, t378: f64, t3154: f64, t12046: f64, t357: f64, t3133: f64, t3302: f64, t4893: f64, t3059: f64, t4975: f64, t4781: f64, t12132: f64, t1647: f64, t3316: f64, t1083: f64, t12122: f64, t12127: f64, t12146: f64, t12149: f64, t12154: f64, t15655: f64, t16529: f64, t16534: f64, t16537: f64, t16540: f64, t16544: f64, t3278: f64, t3288: f64, t3309: f64, t3319: f64, t4954: f64, t4964: f64, t4977: f64, t4981: f64, t4996: f64, t5009: f64, t16423: f64, t16475: f64, t16526: f64, t1079: f64, t1071: f64, t4746: f64, t15669: f64, t379: f64, t994: f64, t1695: f64, t3268: f64, t3066: f64, t1000: f64, t1076: f64, t1097: f64, t11128: f64, t11210: f64, t11214: f64, t16362: f64, t16371: f64, t16374: f64, t1652: f64, t1696: f64, t3047: f64, t3060: f64, t3067: f64, t3076: f64, t3264: f64, t4747: f64, t4773: f64, t4778: f64, t5016: f64, t16272: f64, t16310: f64, t16355: f64, t1100: f64, t1102: f64, t15418: f64, t15420: f64, t15423: f64, t15425: f64, t15427: f64, t15477: f64, t15515: f64, t15549: f64, t15551: f64, t15553: f64, t15555: f64, t15558: f64, t15561: f64, t15562: f64, t15566: f64, t15571: f64, t15575: f64, t15577: f64, t16181: f64, t198: f64, t3333: f64, t336: f64, t5023: f64, t30: f64, t265: f64, t393: f64, t15083: f64, t15546: f64, t1106: f64, t13312: f64, t1468: f64, t1469: f64, t15093: f64, t15094: f64, t15096: f64, t1587: f64, t1704: f64, t2257: f64, t2258: f64, t2838: f64, t3340: f64, t395: f64, t4186: f64, t45: f64, t4560: f64, t5028: f64, t605: f64, t606: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16552, t16554, t16555, t16559, t16561, t16562, t16566, t16568, t16569, t16573) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1637(t16551, t342, t11631, t12050, t3151, t15907, t12077, t378, t3154, t12046, t357, t3133, t3302);
        let (t16577, t16589) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1638(t16573, t4893, t3059, t4975, t4781, t12132, t1647, t3316, t1083, t12122, t12127, t12146, t12149, t12154, t15655, t16529, t16534, t16537, t16540, t16544, t16552, t16555, t16559, t16562, t16566, t16569, t3278, t3288, t3309, t3319, t342, t4954, t4964, t4977, t4981, t4996, t5009);
        let (t16591, t16592, t16597, t16600, t16603, t16604) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1639(t16423, t16475, t16526, t16589, t1079, t1071, t4746, t15669, t378, t379, t994, t1695, t3268);
        let (t16605, t16610) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1640(t16604, t3066, t1000, t1076, t1097, t11128, t11210, t11214, t16362, t16371, t16374, t1652, t16592, t16597, t16600, t16603, t1696, t3047, t3060, t3067, t3076, t3264, t4747, t4773, t4778, t5016);
        let (t16612, t16616) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1641(t16272, t16310, t16355, t16610, t1100, t1102, t15418, t15420, t15423, t15425, t15427, t15477, t15515, t15549, t15551, t15553, t15555, t15558, t15561, t15562, t15566, t15571, t15575, t15577, t16181, t198, t3333, t336, t5023);
        let t16630 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1642(t30, t265, t393, t15083, t15546, t16616, t1106, t13312, t1468, t1469, t15093, t15094, t15096, t1587, t1704, t2257, t2258, t2838, t3340, t395, t4186, t45, t4560, t5028, t605, t606, dens_threshold, rho0, zeta_threshold);
    (t16554, t16561, t16568, t16573, t16577, t16591, t16592, t16605, t16612, t16630)
}
