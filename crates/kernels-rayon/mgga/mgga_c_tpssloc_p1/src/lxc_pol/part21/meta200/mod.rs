//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta200 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1238;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1239;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta200(t1166: f64, t4869: f64, t1703: f64, t3411: f64, t1694: f64, t3375: f64, t1157: f64, t1164: f64, t1147: f64, t1156: f64, t4857: f64, t3400: f64, t1155: f64, t3403: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4871, t4873, t4874, t4875, t4877, t4879, t4881, t4882) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1238(t1166, t4869, t1703, t3411, t1694, t3375, t1157, t1164, t1147, t1156, t4857, t3400);
        let t4883 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1239(t1155, t3403);
    (t4871, t4873, t4874, t4875, t4877, t4879, t4881, t4882, t4883)
}
