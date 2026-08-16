//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta433 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1271;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1272;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta433(t13969: f64, t22270: f64, t3506: f64, t1227: f64, t22257: f64, t21769: f64, t248: f64, t3521: f64, t22157: f64, t3577: f64, t45124: f64, t11697: f64, t22287: f64, t15569: f64, t18371: f64, t19051: f64, t4993: f64, t11784: f64, t21762: f64, t1174: f64, t135: f64, t22128: f64, t22132: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t72470, t72495, t72501, t72512, t72530) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1271(t13969, t22270, t3506, t1227, t22257, t21769, t248, t3521, t22157, t3577, t45124, t11697, t22287);
        let (t72542, t72556, t72560, t72597, t72600) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1272(t15569, t18371, t19051, t4993, t11784, t1227, t21762, t248, t1174, t135, t22128, t22132);
    (t72470, t72495, t72501, t72512, t72530, t72542, t72556, t72560, t72597, t72600)
}
