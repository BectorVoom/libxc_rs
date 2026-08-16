//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta481 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta481<F: Float>(t5499: F, t9929: F, t172: F, t5522: F, t763: F, t184: F, t5398: F, t607: F, t4194: F, t9864: F, t9866: F, t2752: F, t5664: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t16612, t16616, t16618, t16619, t16620, t16622, t16623, t16624, t16625) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2077::<F>(t5499, t9929, t172, t5522, t763, t184, t5398, t607, t4194, t9864, t9866, t2752, t5664);
    (t16612, t16616, t16618, t16619, t16620, t16622, t16623, t16624, t16625)
}
