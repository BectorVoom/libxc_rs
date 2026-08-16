//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta234 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk947;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk948;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk949;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk950;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk951;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk952;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta234(t11129: f64, t1156: f64, t3400: f64, t1164: f64, t268: f64, t405: f64, t6546: f64, t1091: f64, t2394: f64, t3244: f64, t690: f64, t3249: f64, t3253: f64, t154: f64, t3584: f64, t3241: f64, t636: f64, t9288: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11131, t11133, t11135) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk947(t11129, t1156, t3400, t1164, t268, t405, t6546);
        let (t11136, t11137) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk948(t11135, t1091, t2394);
        let t11139 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk949(t3244, t690);
        let t11141 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk950(t3249, t690);
        let t11143 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk951(t3253, t690);
        let (t11145, t11147) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk952(t154, t3584, t3241, t636);
        let t11148 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk953(t11147, t9288);
    (t11131, t11133, t11135, t11136, t11137, t11139, t11141, t11143, t11145, t11147, t11148)
}
