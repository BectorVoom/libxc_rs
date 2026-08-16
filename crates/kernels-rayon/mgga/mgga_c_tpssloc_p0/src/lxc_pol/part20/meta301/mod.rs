//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta301 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1530;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1531;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1532;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1533;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1534;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1535;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1536;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1537;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta301(t11147: f64, t9288: f64, t11145: f64, t123: f64, t3241: f64, t52: f64, t3240: f64, t3242: f64, t607: f64, t2250: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t11148 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1530(t11147, t9288);
        let (t11149, t11150) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1531(t11145, t11148, t123);
        let (t11152, t11153) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1532(t3241, t52);
        let t11154 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1533(t11153, t9288);
        let (t11155, t11156) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1534(t11154, t3240, t123);
        let t11159 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1535(t3242, t607, t2250);
        let (t11160, t11161) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1536(t11159, t3240, t123);
        let t11163 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1537(t3242, t9288);
    (t11148, t11149, t11150, t11152, t11153, t11154, t11155, t11156, t11159, t11160, t11161, t11163)
}
