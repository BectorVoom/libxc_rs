//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1972;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1973;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1974;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1975;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta568(t30886: f64, t7652: f64, t1287: f64, t1794: f64, t29122: f64, t2150: f64, t30840: f64, t473: f64, t1828: f64, t8197: f64, t1775: f64, t1829: f64, t2149: f64, t2152: f64, t26906: f64, t26976: f64, t26994: f64, t29129: f64, t29207: f64, t29220: f64, t29227: f64, t29304: f64, t30867: f64, t30870: f64, t30874: f64, t30878: f64, t30883: f64, t6574: f64, t6580: f64, t6588: f64, t6703: f64, t6745: f64, t7602: f64, t7632: f64, t7636: f64, t7651: f64, t7659: f64, t8213: f64, t265: f64, t502: f64, t30865: f64, t1300: f64, t1832: f64, t198: f64, t27041: f64, t29317: f64, t29930: f64, t336: f64, t5023: f64, t6748: f64, t6752: f64, t7673: f64, t33: f64, t1469: f64, t2159: f64, t29977: f64, t57: f64, t5825: f64, t8227: f64, t30734: f64, t118: f64, t1502: f64, t1843: f64, t1911: f64, t2127: f64, t2163: f64, t29497: f64, t29501: f64, t29504: f64, t29507: f64, t29510: f64, t29512: f64, t29578: f64, t29580: f64, t29582: f64, t29585: f64, t30716: f64, t30724: f64, t508: f64, t5877: f64, t5884: f64, t6765: f64, t8152: f64, t8233: f64, t8237: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t5920: f64, t1518: f64, t29427: f64, t30137: f64, t30140: f64, t30142: f64, t30145: f64, t30147: f64, t30149: f64, t7586: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30887, t30893, t30899, t30906, t30907, t30922) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1972(t30886, t7652, t1287, t1794, t29122, t2150, t30840, t473, t1828, t8197, t1775, t1829, t2149, t2152, t26906, t26976, t26994, t29129, t29207, t29220, t29227, t29304, t30867, t30870, t30874, t30878, t30883, t6574, t6580, t6588, t6703, t6745, t7602, t7632, t7636, t7651, t7659, t8213);
        let (t30923, t30936) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1973(t265, t502, t30865, t30922, t1300, t1832, t198, t27041, t29317, t29930, t336, t5023, t6748, t6752, t7673);
        let (t30944, t30950) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1974(t33, t1469, t2159, t29977, t30936, t57, t5825, t8227, t30734, t118, t1502, t1843, t1911, t2127, t2163, t29497, t29501, t29504, t29507, t29510, t29512, t29578, t29580, t29582, t29585, t30716, t30724, t508, t5877, t5884, t6765, t8152, t8233, t8237, dens_threshold, rho1, zeta_threshold);
        let (t30951, t30959) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1975(t2163, t5920, t1518, t29427, t30137, t30140, t30142, t30145, t30147, t30149, t30716, t30724, t7586);
    (t30887, t30893, t30899, t30906, t30907, t30923, t30936, t30944, t30950, t30951, t30959)
}
