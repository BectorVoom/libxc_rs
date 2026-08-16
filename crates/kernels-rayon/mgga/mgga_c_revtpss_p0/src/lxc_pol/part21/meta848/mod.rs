//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta848 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3186;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3187;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3188;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta848(t1196: f64, t12487: f64, t1756: f64, t45187: f64, t45190: f64, t16784: f64, t3543: f64, t58322: f64, t58325: f64, t58327: f64, t58330: f64, t58333: f64, t58341: f64, t58344: f64, t58462: f64, t58464: f64, t58468: f64, t58658: f64, t58660: f64, t58662: f64, t58664: f64, t58669: f64, t58671: f64, t58675: f64, t58678: f64, t16639: f64, t3531: f64, t3535: f64, t12581: f64, t5192: f64, t12488: f64, t16810: f64, t17150: f64, t3520: f64, t5206: f64, t12494: f64, t16642: f64, t12552: f64, t16811: f64, t5180: f64, t300: f64, t3521: f64, t1757: f64, t58666: f64, t12596: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58683, t58685, t58686) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3186(t1196, t12487, t1756, t45187, t45190, t16784, t3543, t58322, t58325, t58327, t58330, t58333, t58341, t58344, t58462, t58464, t58468, t58658, t58660, t58662, t58664, t58669, t58671, t58675, t58678);
        let (t58688, t58690, t58692, t58695, t58700) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3187(t16639, t3531, t16784, t3535, t12581, t5192, t1196, t12488, t16810, t17150, t3520, t5206);
        let (t58703, t58707, t58711, t58713) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3188(t1196, t12494, t16642, t12552, t16811, t5180, t300, t3521, t1757, t58666, t12596, t5192);
    (t58683, t58685, t58686, t58688, t58690, t58692, t58695, t58700, t58703, t58707, t58711, t58713)
}
