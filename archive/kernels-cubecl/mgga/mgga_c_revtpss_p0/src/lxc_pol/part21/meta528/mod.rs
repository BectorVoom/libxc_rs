//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2170;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2171;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2172;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2173;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta528<F: Float>(t16604: F, t3066: F, t1000: F, t1076: F, t1097: F, t11128: F, t11210: F, t11214: F, t16362: F, t16371: F, t16374: F, t1652: F, t16592: F, t16597: F, t16600: F, t16603: F, t1696: F, t3047: F, t3060: F, t3067: F, t3076: F, t3264: F, t4747: F, t4773: F, t4778: F, t5016: F, t16272: F, t16310: F, t16355: F, t1100: F, t1102: F, t15418: F, t15420: F, t15423: F, t15425: F, t15427: F, t15477: F, t15515: F, t15549: F, t15551: F, t15553: F, t15555: F, t15558: F, t15561: F, t15562: F, t15566: F, t15571: F, t15575: F, t15577: F, t16181: F, t198: F, t3333: F, t336: F, t5023: F, t30: F, t265: F, t393: F, t15083: F, t15546: F, t1106: F, t13312: F, t1468: F, t1469: F, t15093: F, t15094: F, t15096: F, t1587: F, t1704: F, t2257: F, t2258: F, t2838: F, t3340: F, t395: F, t4186: F, t45: F, t4560: F, t5028: F, t605: F, t606: F, dens_threshold: F, rho0: F, zeta_threshold: F, t3498: F, t5205: F, t1196: F, t12485: F, t1756: F, t3524: F, t3531: F, t5198: F, t12361: F, t5068: F, t12243: F, t5109: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16605, t16610) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2170::<F>(t16604, t3066, t1000, t1076, t1097, t11128, t11210, t11214, t16362, t16371, t16374, t1652, t16592, t16597, t16600, t16603, t1696, t3047, t3060, t3067, t3076, t3264, t4747, t4773, t4778, t5016);
        let (t16612, t16616) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2171::<F>(t16272, t16310, t16355, t16610, t1100, t1102, t15418, t15420, t15423, t15425, t15427, t15477, t15515, t15549, t15551, t15553, t15555, t15558, t15561, t15562, t15566, t15571, t15575, t15577, t16181, t198, t3333, t336, t5023);
        let (t16618, t16630) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2172::<F>(t30, t265, t393, t15083, t15546, t16616, t1106, t13312, t1468, t1469, t15093, t15094, t15096, t1587, t1704, t2257, t2258, t2838, t3340, t395, t4186, t45, t4560, t5028, t605, t606, dens_threshold, rho0, zeta_threshold);
        let (t16639, t16641, t16642, t16643, t16645, t16647, t16649, t16651) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2173::<F>(t3498, t5205, t1196, t12485, t1756, t3524, t3531, t5198, t12361, t5068, t12243, t5109);
    (t16605, t16612, t16618, t16630, t16639, t16641, t16642, t16643, t16645, t16647, t16649, t16651)
}
