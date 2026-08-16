//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta505 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1746;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1747;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1748;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1749;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta505(t1842: f64, t7213: f64, t3887: f64, t1807: f64, t7191: f64, t1375: f64, t16460: f64, t1843: f64, t2092: f64, t22908: f64, t22910: f64, t22922: f64, t22928: f64, t22941: f64, t24082: f64, t24156: f64, t24157: f64, t5215: f64, t5321: f64, t5354: f64, t568: f64, t7194: f64, t7199: f64, t7214: f64, t27005: f64, t27065: f64, t27127: f64, t533: f64, t1390: f64, t671: f64, t7890: f64, t2075: f64, t4072: f64, t2039: f64, t5107: f64, t109: f64, t26127: f64, t22472: f64, t23912: f64, t26130: f64, t26132: f64, t510: f64, t1458: f64, t7156: f64, t1983: f64, t2040: f64, t2314: f64, t26179: f64, t4028: f64, t4034: f64, t652: f64, t7050: f64, t7057: f64, t7061: f64, t7458: f64, t7796: f64, t7806: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27132, t27137, t27141) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1746(t1842, t7213, t3887, t1807, t7191, t1375, t16460, t1843, t2092, t22908, t22910, t22922, t22928, t22941, t24082, t24156, t24157, t5215, t5321, t5354, t568, t7194, t7199, t7214);
        let (t27143, t27144, t27145, t27147, t27150, t27163) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1747(t27005, t27065, t27127, t27141, t533, t1390, t671, t7890, t2075, t4072, t2039, t5107);
        let t27170 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1748(t109, t26127, t22472, t23912, t26130, t26132);
        let (t27171, t27180, t27183) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1749(t27170, t510, t1458, t7156, t1983, t2040, t2314, t26179, t27145, t27147, t27150, t27163, t4028, t4034, t652, t7050, t7057, t7061, t7458, t7796, t7806);
    (t27132, t27137, t27143, t27144, t27145, t27147, t27150, t27163, t27170, t27171, t27180, t27183)
}
