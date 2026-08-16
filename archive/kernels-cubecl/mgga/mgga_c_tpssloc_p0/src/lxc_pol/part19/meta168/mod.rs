//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta168 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk797;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk798;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta168<F: Float>(t207: F, t215: F, t9569: F, t2570: F, t782: F, t2573: F, t2690: F, t59: F, t154: F, t2588: F, t21: F, t795: F, t4127: F, t787: F, t9526: F, t9529: F, t9540: F, t9542: F, t9544: F, t9547: F, t9552: F, t9556: F, t9559: F, t9561: F, t9566: F) -> (F, F, F, F, F) {
        let (t9572, t9573, t9574, t9576, t9577, t9579, t9580, t9583) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk797::<F>(t207, t215, t9569, t2570, t782, t2573, t2690, t59, t154, t2588, t21, t795);
        let t9584 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk798::<F>(t4127, t787, t9526, t9529, t9540, t9542, t9544, t9547, t9552, t9556, t9559, t9561, t9566, t9572, t9574, t9579, t9583);
    (t9573, t9576, t9577, t9580, t9584)
}
