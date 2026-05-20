//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1152;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1153;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1154;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1155;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta344<F: Float>(t9597: F, t123: F, t1856: F, t2630: F, t1857: F, t3860: F, t3863: F, t13581: F, t189: F, t512: F, t1907: F, t9593: F, t30: F, t5566: F, t749: F, t9856: F, t1468: F, t9605: F, t2: F, t3874: F, t1344: F, t13554: F, t22: F, t2257: F, t3834: F, t5574: F, t5577: F, t580: F, zeta_threshold: F, t33: F, t1711: F, t9617: F, t3881: F, t1348: F, t13569: F, t3351: F, t3842: F, t5582: F, t5585: F, t1892: F, t785: F, t1358: F, t2439: F, t1903: F, t4075: F, t1444: F, t556: F, t2782: F, t212: F, t5710: F, t689: F, t4131: F, t4076: F, t4077: F, t9657: F, t5774: F, t10171: F, t1424: F, t1904: F, t9632: F, t9636: F, t9639: F, t9642: F, t9650: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13664, t13667, t13669, t13671, t13673, t13674) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1152::<F>(t9597, t123, t1856, t2630, t1857, t3860, t3863, t13581, t189, t512, t1907, t9593);
        let (t13682, t13683, t13700) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1153::<F>(t30, t5566, t749, t512, t9856, t1468, t9605, t2, t3874, t1344, t13554, t22, t2257, t3834, t5574, t5577, t580, zeta_threshold);
        let t13714 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1154::<F>(t33, t1711, t9617, t2, t3881, t1348, t13569, t22, t3351, t3842, t5582, t5585, t580, zeta_threshold);
        let (t13716, t13727, t13733, t13734) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1155::<F>(t13700, t13714, t1892, t785, t1358, t2439, t1903, t4075, t1444, t556, t2782, t212, t5710);
        let t13750 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1156::<F>(t1358, t13734, t689, t1903, t4131, t4076, t4077, t9657, t1444, t5774, t10171, t13727, t13733, t1424, t1904, t9632, t9636, t9639, t9642, t9650);
    (t13664, t13667, t13669, t13671, t13673, t13674, t13682, t13683, t13716, t13750)
}
