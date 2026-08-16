//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2126;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2127;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta567<F: Float>(t1030: F, t10477: F, t10472: F, t10475: F, t3128: F, t10903: F, t10948: F, t10890: F, t10898: F, t3103: F, t10904: F, t11002: F, t10508: F, t248: F, t3130: F, t3132: F, t10969: F, t121: F, t10305: F, t1041: F, t1015: F, t3033: F, t42520: F, t3142: F, t698: F, t973: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42559, t42561, t42565, t42570, t42573, t42578, t42582) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2126::<F>(t1030, t10477, t10472, t10475, t3128, t10903, t10948, t10890, t10898, t3103, t10904, t11002);
        let (t42586, t42595, t42600, t42610) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2127::<F>(t10508, t248, t3130, t3132, t10969, t121, t10305, t1041, t1015, t3033, t42520, t3142, t698, t973);
    (t42559, t42561, t42565, t42570, t42573, t42578, t42582, t42586, t42595, t42600, t42610)
}
