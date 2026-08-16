//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta342 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1542;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta342<F: Float>(t13109: F, t13113: F, t5398: F, t751: F, t707: F, t13133: F, t1462: F, t2427: F, t5597: F, t9922: F, t13124: F, t5522: F, t67: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t16699, t16700, t16701, t16702, t16703, t16705, t16707, t16708, t16709, t16710) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1542::<F>(t13109, t13113, t5398, t751, t707, t13133, t1462, t2427, t5597, t9922, t13124, t5522, t67);
    (t16699, t16700, t16701, t16702, t16703, t16705, t16707, t16708, t16709, t16710)
}
