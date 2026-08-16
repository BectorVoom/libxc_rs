//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta232 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1301;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1302;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta232<F: Float>(t761: F, t9722: F, t2517: F, t718: F, t2475: F, t723: F, t159: F, t2461: F, t730: F, t167: F, t2478: F, t164: F, t2479: F, t9689: F, t9692: F, t9695: F, t9698: F, t9702: F, t9704: F, t9706: F, t9709: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9724, t9726, t9729, t9730, t9731, t9733, t9734, t9738) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1301::<F>(t761, t9722, t2517, t718, t2475, t723, t159, t2461, t730, t167, t2478, t164);
        let (t9739, t9740, t9751) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1302::<F>(t159, t9738, t2479, t9731, t9689, t9692, t9695, t9698, t9702, t9704, t9706, t9709);
    (t9724, t9726, t9729, t9730, t9731, t9733, t9734, t9738, t9739, t9740, t9751)
}
