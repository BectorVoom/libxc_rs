//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1779;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1780;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1781;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1782;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1783;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1784;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1785;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta466<F: Float>(t10162: F, t9303: F, t3903: F, t9292: F, t1445: F, t2439: F, t9640: F, t3906: F, t3907: F, t39494: F, t1426: F, t4067: F, t786: F, t3917: F, t2453: F, t3908: F, t10115: F, t1421: F, t10168: F, t3920: F, t10147: F, t4071: F, t47472: F, t47474: F, t47478: F, t47483: F, t47487: F, t47490: F, t47493: F, t10174: F, t9676: F, t123: F, t2434: F, t3915: F, t4131: F, t10175: F, t9686: F, t1420: F, t4075: F, t9682: F, t3895: F, t4132: F, t1357: F, t689: F, t9659: F, t3899: F, t9671: F, t10146: F, t676: F, t10008: F, t1358: F, t212: F, t1359: F, t39501: F, t10171: F, t1424: F, t4076: F, t4077: F, t9657: F, t555: F, t10165: F, t9664: F, t1427: F, t1444: F, t22: F, t9647: F, t9680: F, t125: F, t8779: F, t9645: F, t9634: F, t2435: F, t9667: F, t268: F, t39644: F, t556: F, t561: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t47495, t47497, t47500, t47504, t47506) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1779::<F>(t10162, t9303, t3903, t9292, t1445, t2439, t9640, t3906, t3907, t39494, t1426, t4067, t786);
        let t47518 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1780::<F>(t3917, t47506, t2453, t3908, t4067, t10115, t1421, t10168, t3920, t10147, t4071, t47472, t47474, t47478, t47483, t47487, t47490, t47493, t47495, t47497, t47500, t47504);
        let (t47521, t47525, t47527, t47530) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1781::<F>(t10174, t2453, t9676, t123, t2434, t3915, t4131, t10175, t9686, t1420, t4075, t786);
        let (t47531, t47534, t47537, t47540, t47546, t47550) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1782::<F>(t47530, t9682, t2439, t3895, t4132, t1357, t689, t9659, t3899, t4131, t10175, t9671);
        let t47566 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1783::<F>(t10146, t123, t3915, t676, t10008, t1358, t212, t689, t1359, t39501, t10171, t1424, t4071, t4076, t4077, t4131, t4132, t47521, t47525, t47527, t47531, t47534, t47537, t47540, t47546, t47550, t9657, t9659);
        let (t47568, t47570, t47574, t47580) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1784::<F>(t10115, t555, t1445, t10165, t9664, t1427, t1444, t22, t9647, t123, t2434, t4077, t9680);
        let (t47591, t47593, t47595, t47601) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1785::<F>(t123, t125, t1358, t555, t8779, t9645, t1445, t689, t9634, t2435, t9667, t268, t39644, t556, t561);
    (t47518, t47566, t47568, t47570, t47574, t47580, t47591, t47593, t47595, t47601)
}
