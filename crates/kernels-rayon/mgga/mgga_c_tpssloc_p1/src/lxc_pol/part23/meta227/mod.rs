//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta227 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk875;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk876;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta227(t16046: f64, t544: f64, t12189: f64, t1804: f64, t5194: f64, t782: f64, t3732: f64, t67: f64, t792: f64, t1799: f64, t212: f64, t12214: f64, t131: f64, t205: f64, t12199: f64, t5202: f64, t12225: f64, t2586: f64, t2371: f64, t5154: f64, t12365: f64, t1827: f64, t12418: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16047, t16078, t16081, t16094, t16095, t16100) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk875(t16046, t544, t12189, t1804, t5194, t782, t3732, t67, t792, t1799, t212, t12214, t131);
        let (t16101, t16108, t16118, t16119, t16164, t16211, t16224) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk876(t16100, t205, t12199, t5202, t12225, t16095, t2586, t2371, t5154, t12365, t1827, t12418, t820);
    (t16047, t16078, t16081, t16094, t16095, t16101, t16108, t16118, t16119, t16164, t16211, t16224)
}
