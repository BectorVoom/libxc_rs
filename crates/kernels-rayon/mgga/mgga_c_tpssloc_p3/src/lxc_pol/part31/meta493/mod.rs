//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1681;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1682;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1683;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1684;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1685;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta493(t27005: f64, t27065: f64, t27127: f64, t27141: f64, t533: f64, t1390: f64, t671: f64, t7890: f64, t2075: f64, t4072: f64, t2039: f64, t5107: f64, t109: f64, t26127: f64, t22472: f64, t23912: f64, t26130: f64, t26132: f64, t510: f64, t1458: f64, t7156: f64, t1983: f64, t2040: f64, t2314: f64, t26179: f64, t4028: f64, t4034: f64, t652: f64, t7050: f64, t7057: f64, t7061: f64, t7458: f64, t7796: f64, t7806: f64, t111: f64, t7786: f64, t1268: f64, t12725: f64, t19456: f64, t23938: f64, t26114: f64, t26117: f64, t26967: f64, t26977: f64, t5113: f64, t7042: f64, t7056: f64, t7676: f64, t7801: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27143, t27144, t27145, t27147, t27150, t27163) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1681(t27005, t27065, t27127, t27141, t533, t1390, t671, t7890, t2075, t4072, t2039, t5107);
        let t27170 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1682(t109, t26127, t22472, t23912, t26130, t26132);
        let (t27171, t27180, t27183) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1683(t27170, t510, t1458, t7156, t1983, t2040, t2314, t26179, t27145, t27147, t27150, t27163, t4028, t4034, t652, t7050, t7057, t7061, t7458, t7796, t7806);
        let t27188 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1684(t111, t7786);
        let t27215 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1685(t1268, t12725, t1458, t19456, t2039, t2314, t23938, t26114, t26117, t26967, t26977, t27170, t27188, t4028, t4072, t5113, t671, t7042, t7056, t7676, t7801);
    (t27143, t27144, t27145, t27147, t27150, t27163, t27170, t27171, t27180, t27183, t27188, t27215)
}
