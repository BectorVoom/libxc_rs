//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta656 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2455;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2456;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2457;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta656<F: Float>(t11545: F, t241: F, t3241: F, t242: F, t281: F, t415: F, t2394: F, t3253: F, t3249: F, t2296: F, t11778: F, t154: F, t1091: F, t9698: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t43761, t43763, t43776, t43777, t43780) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2455::<F>(t11545, t241, t3241, t242, t281, t415, t2394, t3253);
        let t43782 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2456::<F>(t2394, t3249);
        let (t43791, t43809, t43816) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2457::<F>(t2296, t3241, t11778, t154, t1091, t9698);
    (t43761, t43763, t43776, t43777, t43780, t43782, t43791, t43809, t43816)
}
