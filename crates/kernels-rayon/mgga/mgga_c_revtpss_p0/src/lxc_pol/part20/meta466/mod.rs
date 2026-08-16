//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1779;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1780;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1781;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1782;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1783;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1784;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1785;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta466(t10162: f64, t9303: f64, t3903: f64, t9292: f64, t1445: f64, t2439: f64, t9640: f64, t3906: f64, t3907: f64, t39494: f64, t1426: f64, t4067: f64, t786: f64, t3917: f64, t2453: f64, t3908: f64, t10115: f64, t1421: f64, t10168: f64, t3920: f64, t10147: f64, t4071: f64, t47472: f64, t47474: f64, t47478: f64, t47483: f64, t47487: f64, t47490: f64, t47493: f64, t10174: f64, t9676: f64, t123: f64, t2434: f64, t3915: f64, t4131: f64, t10175: f64, t9686: f64, t1420: f64, t4075: f64, t9682: f64, t3895: f64, t4132: f64, t1357: f64, t689: f64, t9659: f64, t3899: f64, t9671: f64, t10146: f64, t676: f64, t10008: f64, t1358: f64, t212: f64, t1359: f64, t39501: f64, t10171: f64, t1424: f64, t4076: f64, t4077: f64, t9657: f64, t555: f64, t10165: f64, t9664: f64, t1427: f64, t1444: f64, t22: f64, t9647: f64, t9680: f64, t125: f64, t8779: f64, t9645: f64, t9634: f64, t2435: f64, t9667: f64, t268: f64, t39644: f64, t556: f64, t561: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47495, t47497, t47500, t47504, t47506) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1779(t10162, t9303, t3903, t9292, t1445, t2439, t9640, t3906, t3907, t39494, t1426, t4067, t786);
        let t47518 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1780(t3917, t47506, t2453, t3908, t4067, t10115, t1421, t10168, t3920, t10147, t4071, t47472, t47474, t47478, t47483, t47487, t47490, t47493, t47495, t47497, t47500, t47504);
        let (t47521, t47525, t47527, t47530) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1781(t10174, t2453, t9676, t123, t2434, t3915, t4131, t10175, t9686, t1420, t4075, t786);
        let (t47531, t47534, t47537, t47540, t47546, t47550) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1782(t47530, t9682, t2439, t3895, t4132, t1357, t689, t9659, t3899, t4131, t10175, t9671);
        let t47566 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1783(t10146, t123, t3915, t676, t10008, t1358, t212, t689, t1359, t39501, t10171, t1424, t4071, t4076, t4077, t4131, t4132, t47521, t47525, t47527, t47531, t47534, t47537, t47540, t47546, t47550, t9657, t9659);
        let (t47568, t47570, t47574, t47580) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1784(t10115, t555, t1445, t10165, t9664, t1427, t1444, t22, t9647, t123, t2434, t4077, t9680);
        let (t47591, t47593, t47595, t47601) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1785(t123, t125, t1358, t555, t8779, t9645, t1445, t689, t9634, t2435, t9667, t268, t39644, t556, t561);
    (t47518, t47566, t47568, t47570, t47574, t47580, t47591, t47593, t47595, t47601)
}
