//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta97 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk663;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk664;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk665;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk666;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk667;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk668;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk669;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta97<F: Float>(t2535: F, t761: F, t718: F, t751: F, t15: F, t60: F, t59: F, t207: F, t215: F, t782: F, t786: F, t789: F, t591: F, t795: F, t154: F, t244: F, t205: F, t792: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2537, t2538, t2558) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk663::<F>(t2535, t761, t718, t751, t15, t60);
        let t2559 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk664::<F>(t2558, t59);
        let (t2562, t2563) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk665::<F>(t207, t215, t2559, t782, t786);
        let (t2564, t2566) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk666::<F>(t2563, t789, t59, t591);
        let (t2569, t2570) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk667::<F>(t207, t2566, t795, t154, t244);
        let t2571 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk668::<F>(t205, t2570);
        let t2576 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk669::<F>(t786, t792);
    (t2537, t2538, t2558, t2559, t2562, t2563, t2564, t2566, t2569, t2570, t2571, t2576)
}
