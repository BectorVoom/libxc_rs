//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2114;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta558<F: Float>(t10770: F, t919: F, t2897: F, t2904: F, t10701: F, t888: F, t275: F, t2790: F, t2840: F, t41654: F, t41961: F, t2843: F) -> (F, F, F, F, F, F, F, F) {
        let (t41984, t42020, t42023, t42028, t42086, t42087, t42100, t42101) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2114::<F>(t10770, t919, t2897, t2904, t10701, t888, t275, t2790, t2840, t41654, t41961, t2843);
    (t41984, t42020, t42023, t42028, t42086, t42087, t42100, t42101)
}
