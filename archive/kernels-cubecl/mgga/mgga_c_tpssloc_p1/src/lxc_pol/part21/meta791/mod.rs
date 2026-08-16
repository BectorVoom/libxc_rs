//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta791 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2751;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta791<F: Float>(t40722: F, t12939: F, t16619: F, t2244: F, t46234: F, t46236: F, t40729: F, t40733: F, t2517: F, t5398: F, t707: F, t10130: F, t12935: F, t193: F, t39472: F, t39476: F, t40721: F, t40732: F, t5527: F, t5544: F) -> (F, F, F, F, F, F, F, F) {
        let (t57983, t57986, t57987, t57988, t57989, t57990, t57993, t57994) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2751::<F>(t40722, t12939, t16619, t2244, t46234, t46236, t40729, t40733, t2517, t5398, t707, t10130, t12935, t193, t39472, t39476, t40721, t40732, t5527, t5544);
    (t57983, t57986, t57987, t57988, t57989, t57990, t57993, t57994)
}
