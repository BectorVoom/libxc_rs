//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta235 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk954;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk955;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk956;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk957;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk958;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk959;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk960;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta235(t11145: f64, t11148: f64, t123: f64, t3241: f64, t52: f64, t9288: f64, t3240: f64, t3242: f64, t607: f64, t2250: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11149, t11150) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk954(t11145, t11148, t123);
        let (t11152, t11153) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk955(t3241, t52);
        let t11154 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk956(t11153, t9288);
        let (t11155, t11156) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk957(t11154, t3240, t123);
        let (t11158, t11159) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk958(t3242, t607, t2250);
        let (t11160, t11161) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk959(t11159, t3240, t123);
        let t11163 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk960(t3242, t9288);
    (t11149, t11150, t11152, t11153, t11154, t11155, t11156, t11158, t11159, t11160, t11161, t11163)
}
