//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta493 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1681;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1682;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1683;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1684;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1685;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta493<F: Float>(t27005: F, t27065: F, t27127: F, t27141: F, t533: F, t1390: F, t671: F, t7890: F, t2075: F, t4072: F, t2039: F, t5107: F, t109: F, t26127: F, t22472: F, t23912: F, t26130: F, t26132: F, t510: F, t1458: F, t7156: F, t1983: F, t2040: F, t2314: F, t26179: F, t4028: F, t4034: F, t652: F, t7050: F, t7057: F, t7061: F, t7458: F, t7796: F, t7806: F, t111: F, t7786: F, t1268: F, t12725: F, t19456: F, t23938: F, t26114: F, t26117: F, t26967: F, t26977: F, t5113: F, t7042: F, t7056: F, t7676: F, t7801: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27143, t27144, t27145, t27147, t27150, t27163) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1681::<F>(t27005, t27065, t27127, t27141, t533, t1390, t671, t7890, t2075, t4072, t2039, t5107);
        let t27170 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1682::<F>(t109, t26127, t22472, t23912, t26130, t26132);
        let (t27171, t27180, t27183) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1683::<F>(t27170, t510, t1458, t7156, t1983, t2040, t2314, t26179, t27145, t27147, t27150, t27163, t4028, t4034, t652, t7050, t7057, t7061, t7458, t7796, t7806);
        let t27188 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1684::<F>(t111, t7786);
        let t27215 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1685::<F>(t1268, t12725, t1458, t19456, t2039, t2314, t23938, t26114, t26117, t26967, t26977, t27170, t27188, t4028, t4072, t5113, t671, t7042, t7056, t7676, t7801);
    (t27143, t27144, t27145, t27147, t27150, t27163, t27170, t27171, t27180, t27183, t27188, t27215)
}
