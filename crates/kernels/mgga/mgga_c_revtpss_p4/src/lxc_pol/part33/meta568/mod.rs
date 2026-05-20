//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1972;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1973;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1974;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1975;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta568<F: Float>(t30886: F, t7652: F, t1287: F, t1794: F, t29122: F, t2150: F, t30840: F, t473: F, t1828: F, t8197: F, t1775: F, t1829: F, t2149: F, t2152: F, t26906: F, t26976: F, t26994: F, t29129: F, t29207: F, t29220: F, t29227: F, t29304: F, t30867: F, t30870: F, t30874: F, t30878: F, t30883: F, t6574: F, t6580: F, t6588: F, t6703: F, t6745: F, t7602: F, t7632: F, t7636: F, t7651: F, t7659: F, t8213: F, t265: F, t502: F, t30865: F, t1300: F, t1832: F, t198: F, t27041: F, t29317: F, t29930: F, t336: F, t5023: F, t6748: F, t6752: F, t7673: F, t33: F, t1469: F, t2159: F, t29977: F, t57: F, t5825: F, t8227: F, t30734: F, t118: F, t1502: F, t1843: F, t1911: F, t2127: F, t2163: F, t29497: F, t29501: F, t29504: F, t29507: F, t29510: F, t29512: F, t29578: F, t29580: F, t29582: F, t29585: F, t30716: F, t30724: F, t508: F, t5877: F, t5884: F, t6765: F, t8152: F, t8233: F, t8237: F, dens_threshold: F, rho1: F, zeta_threshold: F, t5920: F, t1518: F, t29427: F, t30137: F, t30140: F, t30142: F, t30145: F, t30147: F, t30149: F, t7586: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t30887, t30893, t30899, t30906, t30907, t30922) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1972::<F>(t30886, t7652, t1287, t1794, t29122, t2150, t30840, t473, t1828, t8197, t1775, t1829, t2149, t2152, t26906, t26976, t26994, t29129, t29207, t29220, t29227, t29304, t30867, t30870, t30874, t30878, t30883, t6574, t6580, t6588, t6703, t6745, t7602, t7632, t7636, t7651, t7659, t8213);
        let (t30923, t30936) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1973::<F>(t265, t502, t30865, t30922, t1300, t1832, t198, t27041, t29317, t29930, t336, t5023, t6748, t6752, t7673);
        let (t30944, t30950) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1974::<F>(t33, t1469, t2159, t29977, t30936, t57, t5825, t8227, t30734, t118, t1502, t1843, t1911, t2127, t2163, t29497, t29501, t29504, t29507, t29510, t29512, t29578, t29580, t29582, t29585, t30716, t30724, t508, t5877, t5884, t6765, t8152, t8233, t8237, dens_threshold, rho1, zeta_threshold);
        let (t30951, t30959) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1975::<F>(t2163, t5920, t1518, t29427, t30137, t30140, t30142, t30145, t30147, t30149, t30716, t30724, t7586);
    (t30887, t30893, t30899, t30906, t30907, t30923, t30936, t30944, t30950, t30951, t30959)
}
