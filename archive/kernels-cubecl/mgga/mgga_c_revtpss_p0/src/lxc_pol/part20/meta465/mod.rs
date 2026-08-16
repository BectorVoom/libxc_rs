//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1776;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1777;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1778;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta465<F: Float>(t138: F, t2438: F, t4131: F, t9674: F, t10059: F, t10090: F, t10130: F, t10146: F, t10171: F, t1399: F, t14193: F, t1424: F, t1427: F, t1437: F, t1444: F, t1445: F, t4057: F, t4076: F, t4078: F, t4087: F, t46350: F, t46353: F, t46356: F, t46359: F, t46362: F, t46363: F, t46368: F, t46369: F, t46378: F, t46381: F, t46385: F, t46388: F, t46392: F, t46394: F, t46398: F, t46401: F, t46403: F, t46407: F, t46412: F, t46416: F, t46424: F, t46467: F, t46472: F, t46476: F, t46479: F, t46483: F, t46490: F, t46493: F, t46496: F, t46500: F, t46505: F, t46510: F, t46515: F, t46551: F, t46554: F, t46561: F, t46563: F, t46568: F, t46570: F, t46572: F, t46574: F, t46583: F, t46587: F, t47383: F, t47418: F, t47457: F, t5675: F, t5745: F, t5755: F, t820: F, t9995: F, t2782: F, t4075: F, t556: F, t3911: F, t9692: F, t123: F, t3915: F, t9291: F, t2453: F, t9679: F, t4077: F, t9302: F, t10009: F, t1364: F, t786: F, t3899: F, t689: F) -> (F, F, F, F, F, F, F, F) {
        let t47468 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1776::<F>(t138, t2438, t4131, t9674, t10059, t10090, t10130, t10146, t10171, t1399, t14193, t1424, t1427, t1437, t1444, t1445, t4057, t4076, t4078, t4087, t46350, t46353, t46356, t46359, t46362, t46363, t46368, t46369, t46378, t46381, t46385, t46388, t46392, t46394, t46398, t46401, t46403, t46407, t46412, t46416, t46424, t46467, t46472, t46476, t46479, t46483, t46490, t46493, t46496, t46500, t46505, t46510, t46515, t46551, t46554, t46561, t46563, t46568, t46570, t46572, t46574, t46583, t46587, t47383, t47418, t47457, t5675, t5745, t5755, t820, t9995);
        let (t47472, t47474, t47478, t47480) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1777::<F>(t1444, t2782, t4075, t4131, t556, t3911, t9692, t123, t3915, t9291, t2453, t9679);
        let (t47483, t47487, t47490, t47493) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1778::<F>(t138, t2438, t4077, t47480, t1444, t9302, t9674, t10009, t1364, t786, t3899, t4078, t689);
    (t47468, t47472, t47474, t47478, t47483, t47487, t47490, t47493)
}
