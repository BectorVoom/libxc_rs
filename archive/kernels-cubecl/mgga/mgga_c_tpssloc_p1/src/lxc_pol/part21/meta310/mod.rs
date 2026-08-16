//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta310 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1660;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1661;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1662;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1663;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1664;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1665;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta310<F: Float>(t2393: F, t374: F, t486: F, t485: F, t248: F, t3516: F, t3570: F, t3515: F, t3576: F, t3604: F, t3585: F, t820: F, t10401: F, t3575: F, t3610: F, t3624: F, t3521: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11647, t11649, t11651, t11652, t11665) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1660::<F>(t2393, t374, t486, t485, t248, t3516, t3570, t3515, t3576, t3604);
        let t11668 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1661::<F>(t3585, t820);
        let t11677 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1662::<F>(t10401, t3575);
        let t11678 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1663::<F>(t11677, t3610);
        let t11692 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1664::<F>(t11677, t3624);
        let t11697 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1665::<F>(t3521, t820);
    (t11647, t11649, t11651, t11652, t11665, t11668, t11677, t11678, t11692, t11697)
}
