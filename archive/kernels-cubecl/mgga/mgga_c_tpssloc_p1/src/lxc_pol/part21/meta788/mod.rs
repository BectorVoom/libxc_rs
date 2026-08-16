//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta788 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2745;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2746;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta788<F: Float>(t46134: F, t46137: F, t4303: F, t776: F, t2517: F, t5520: F, t40667: F, t40673: F, t40680: F, t2522: F, t39309: F, t39312: F, t39316: F, t39320: F, t40679: F, t4307: F, t40682: F, t40687: F, t46196: F, t1484: F, t2752: F, t13487: F, t2749: F, t12854: F, t12915: F, t13196: F, t1530: F, t16596: F, t16944: F, t17116: F, t17120: F, t1877: F, t193: F, t200: F, t2523: F, t2745: F, t39373: F, t40685: F, t4310: F, t4314: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t57891, t57892, t57897, t57898, t57899, t57900, t57901) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2745::<F>(t46134, t46137, t4303, t776, t2517, t5520, t40667, t40673, t40680, t2522, t39309, t39312, t39316, t39320, t40679, t4307);
        let (t57903, t57907, t57908, t57931) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2746::<F>(t40682, t40687, t46196, t1484, t2752, t13487, t2749, t12854, t12915, t13196, t1530, t16596, t16944, t17116, t17120, t1877, t193, t200, t2522, t2523, t2745, t39373, t40685, t4310, t4314);
    (t57891, t57892, t57897, t57898, t57899, t57900, t57901, t57903, t57907, t57908, t57931)
}
