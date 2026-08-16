//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2351;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2352;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta599<F: Float>(t11152: F, t76: F, t41: F, t42: F, t53: F, t54: F, t9576: F, t2405: F, t2420: F, t702: F, t2412: F, t125: F, t2409: F, t2418: F, t9481: F) -> (F, F, F, F, F, F, F, F) {
        let (t39114, t39159, t39168, t39210, t39246, t39249) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2351::<F>(t11152, t76, t41, t42, t53, t54, t9576, t2405, t2420, t702);
        let (t39253, t39256) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2352::<F>(t2412, t125, t2409, t2418, t9481);
    (t39114, t39159, t39168, t39210, t39246, t39249, t39253, t39256)
}
