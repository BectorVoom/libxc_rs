//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1511;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1512;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1513;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1514;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta264<F: Float>(t9489: F, t9490: F, t9493: F, t761: F, t116: F, t229: F, t212: F, t776: F, t2586: F, t597: F, t60: F, t59: F, t2386: F, t131: F, t207: F, t2559: F, t786: F, t789: F, t2563: F, t2582: F, t2566: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t9494 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1511::<F>(t9489, t9490, t9493);
        let (t9496, t9523) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1512::<F>(t761, t9494, t116, t229);
        let (t9525, t9526, t9534, t9538, t9540, t9541) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1513::<F>(t212, t776, t9523, t2586, t597, t60, t59, t2386, t116, t131, t207, t2559, t786);
        let (t9542, t9544, t9546) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1514::<F>(t789, t9541, t2563, t2582, t2566, t786);
    (t9494, t9496, t9523, t9525, t9526, t9534, t9538, t9540, t9541, t9542, t9544, t9546)
}
