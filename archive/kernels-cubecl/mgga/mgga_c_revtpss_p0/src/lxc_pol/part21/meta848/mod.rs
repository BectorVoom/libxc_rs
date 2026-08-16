//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta848 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3186;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3187;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3188;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta848<F: Float>(t1196: F, t12487: F, t1756: F, t45187: F, t45190: F, t16784: F, t3543: F, t58322: F, t58325: F, t58327: F, t58330: F, t58333: F, t58341: F, t58344: F, t58462: F, t58464: F, t58468: F, t58658: F, t58660: F, t58662: F, t58664: F, t58669: F, t58671: F, t58675: F, t58678: F, t16639: F, t3531: F, t3535: F, t12581: F, t5192: F, t12488: F, t16810: F, t17150: F, t3520: F, t5206: F, t12494: F, t16642: F, t12552: F, t16811: F, t5180: F, t300: F, t3521: F, t1757: F, t58666: F, t12596: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t58683, t58685, t58686) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3186::<F>(t1196, t12487, t1756, t45187, t45190, t16784, t3543, t58322, t58325, t58327, t58330, t58333, t58341, t58344, t58462, t58464, t58468, t58658, t58660, t58662, t58664, t58669, t58671, t58675, t58678);
        let (t58688, t58690, t58692, t58695, t58700) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3187::<F>(t16639, t3531, t16784, t3535, t12581, t5192, t1196, t12488, t16810, t17150, t3520, t5206);
        let (t58703, t58707, t58711, t58713) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3188::<F>(t1196, t12494, t16642, t12552, t16811, t5180, t300, t3521, t1757, t58666, t12596, t5192);
    (t58683, t58685, t58686, t58688, t58690, t58692, t58695, t58700, t58703, t58707, t58711, t58713)
}
