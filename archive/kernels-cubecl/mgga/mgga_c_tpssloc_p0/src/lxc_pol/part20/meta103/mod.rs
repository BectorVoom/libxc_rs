//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta103 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk691;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk692;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk693;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk694;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk695;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk696;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta103<F: Float>(t2563: F, t789: F, t59: F, t591: F, t207: F, t795: F, t154: F, t244: F, t205: F, t210: F, t214: F, t2379: F, t786: F, t792: F, t118: F, t776: F, t794: F, t2553: F, t835: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2564, t2566) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk691::<F>(t2563, t789, t59, t591);
        let (t2569, t2570) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk692::<F>(t207, t2566, t795, t154, t244);
        let t2571 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk693::<F>(t205, t2570);
        let (t2573, t2576) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk694::<F>(t210, t214, t2379, t786, t792);
        let (t2578, t2579, t2582, t2585) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk695::<F>(t118, t776, t794, t2576, t210, t214, t2553, t59, t835);
        let t2586 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk696::<F>(t154, t2585);
    (t2564, t2566, t2569, t2570, t2571, t2573, t2576, t2578, t2579, t2582, t2585, t2586)
}
