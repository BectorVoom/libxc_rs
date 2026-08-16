//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta346 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1742;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1743;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta346<F: Float>(t12984: F, t9523: F, t2586: F, t213: F, t4119: F, t221: F, t776: F, t2553: F, t4128: F, t2570: F, t67: F, t792: F, t686: F, t4127: F, t9526: F, t9540: F, t9542: F, t9544: F, t9547: F, t9552: F, t9556: F, t131: F, t9558: F, t205: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12985, t12986, t12990, t12994, t12997, t12998) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1742::<F>(t12984, t9523, t2586, t213, t4119, t221, t776, t2553, t4128, t2570, t67, t792);
        let (t13000, t13002, t13003) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1743::<F>(t12984, t686, t776, t12998, t12986, t12990, t12994, t4127, t9526, t9540, t9542, t9544, t9547, t9552, t9556);
        let (t13004, t13005) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1744::<F>(t131, t9558, t205);
    (t12985, t12986, t12990, t12994, t12997, t12998, t13000, t13002, t13003, t13004, t13005)
}
