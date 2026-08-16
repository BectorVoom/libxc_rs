//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta300 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1630;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1631;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1632;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1633;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1634;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1635;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1636;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1637;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta300<F: Float>(t11135: F, t1091: F, t2394: F, t3244: F, t690: F, t3249: F, t3253: F, t154: F, t3584: F, t3241: F, t636: F, t52: F, t1098: F, t3256: F, t1094: F, t3312: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11136, t11137) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1630::<F>(t11135, t1091, t2394);
        let t11139 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1631::<F>(t3244, t690);
        let t11141 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1632::<F>(t3249, t690);
        let t11143 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1633::<F>(t3253, t690);
        let t11145 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1634::<F>(t154, t3584);
        let t11147 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1635::<F>(t3241, t636);
        let (t11152, t11153) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1636::<F>(t3241, t52);
        let (t11180, t11185) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1637::<F>(t1098, t3256, t1094, t3312);
    (t11136, t11137, t11139, t11141, t11143, t11145, t11147, t11152, t11153, t11180, t11185)
}
