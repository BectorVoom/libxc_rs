//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta315 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1576;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta315<F: Float>(t11496: F, t457: F, t460: F, t974: F, t1184: F, t3475: F, t3469: F, t4934: F, t135: F, t3477: F, t1174: F, t11153: F, t461: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11498, t11499, t11502, t11504, t11505, t11509, t11510, t11513, t11514, t11516) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1576::<F>(t11496, t457, t460, t974, t1184, t3475, t3469, t4934, t135, t3477, t1174, t11153, t461);
    (t11498, t11499, t11502, t11504, t11505, t11509, t11510, t11513, t11514, t11516)
}
