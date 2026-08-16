//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta369 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1169;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta369<F: Float>(t44620: F, t974: F, t43763: F, t461: F, t1176: F, t2402: F, t42339: F, t466: F, t11715: F, t42341: F, t11721: F, t23508: F) -> (F, F, F, F, F, F) {
        let (t44621, t44622, t44633, t44696, t44698, t44701) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1169::<F>(t44620, t974, t43763, t461, t1176, t2402, t42339, t466, t11715, t42341, t11721, t23508);
    (t44621, t44622, t44633, t44696, t44698, t44701)
}
