//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta661 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2462;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta661<F: Float>(t11282: F, t1143: F, t43689: F, t440: F, t43776: F, t43819: F, t3324: F, t3356: F, t3330: F, t3355: F, t427: F, t1174: F, t3471: F, t698: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t44220, t44223, t44249, t44275, t44300, t44320, t44348, t44361, t44424) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2462::<F>(t11282, t1143, t43689, t440, t43776, t43819, t3324, t3356, t3330, t3355, t427, t1174, t3471, t698);
    (t44220, t44223, t44249, t44275, t44300, t44320, t44348, t44361, t44424)
}
