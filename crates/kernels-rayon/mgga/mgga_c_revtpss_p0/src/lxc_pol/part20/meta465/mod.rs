//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1776;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1777;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1778;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta465(t138: f64, t2438: f64, t4131: f64, t9674: f64, t10059: f64, t10090: f64, t10130: f64, t10146: f64, t10171: f64, t1399: f64, t14193: f64, t1424: f64, t1427: f64, t1437: f64, t1444: f64, t1445: f64, t4057: f64, t4076: f64, t4078: f64, t4087: f64, t46350: f64, t46353: f64, t46356: f64, t46359: f64, t46362: f64, t46363: f64, t46368: f64, t46369: f64, t46378: f64, t46381: f64, t46385: f64, t46388: f64, t46392: f64, t46394: f64, t46398: f64, t46401: f64, t46403: f64, t46407: f64, t46412: f64, t46416: f64, t46424: f64, t46467: f64, t46472: f64, t46476: f64, t46479: f64, t46483: f64, t46490: f64, t46493: f64, t46496: f64, t46500: f64, t46505: f64, t46510: f64, t46515: f64, t46551: f64, t46554: f64, t46561: f64, t46563: f64, t46568: f64, t46570: f64, t46572: f64, t46574: f64, t46583: f64, t46587: f64, t47383: f64, t47418: f64, t47457: f64, t5675: f64, t5745: f64, t5755: f64, t820: f64, t9995: f64, t2782: f64, t4075: f64, t556: f64, t3911: f64, t9692: f64, t123: f64, t3915: f64, t9291: f64, t2453: f64, t9679: f64, t4077: f64, t9302: f64, t10009: f64, t1364: f64, t786: f64, t3899: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t47468 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1776(t138, t2438, t4131, t9674, t10059, t10090, t10130, t10146, t10171, t1399, t14193, t1424, t1427, t1437, t1444, t1445, t4057, t4076, t4078, t4087, t46350, t46353, t46356, t46359, t46362, t46363, t46368, t46369, t46378, t46381, t46385, t46388, t46392, t46394, t46398, t46401, t46403, t46407, t46412, t46416, t46424, t46467, t46472, t46476, t46479, t46483, t46490, t46493, t46496, t46500, t46505, t46510, t46515, t46551, t46554, t46561, t46563, t46568, t46570, t46572, t46574, t46583, t46587, t47383, t47418, t47457, t5675, t5745, t5755, t820, t9995);
        let (t47472, t47474, t47478, t47480) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1777(t1444, t2782, t4075, t4131, t556, t3911, t9692, t123, t3915, t9291, t2453, t9679);
        let (t47483, t47487, t47490, t47493) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1778(t138, t2438, t4077, t47480, t1444, t9302, t9674, t10009, t1364, t786, t3899, t4078, t689);
    (t47468, t47472, t47474, t47478, t47483, t47487, t47490, t47493)
}
