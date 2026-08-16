//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta175 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk813;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk814;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk815;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta175<F: Float>(t761: F, t9713: F, t172: F, t2448: F, t763: F, t177: F, t2508: F, t2512: F, t9490: F, t9450: F, t9457: F, t9463: F, t9469: F, t9476: F, t9484: F, t9496: F, t9684: F) -> (F, F, F, F, F, F, F) {
        let (t9715, t9716, t9718, t9720) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk813::<F>(t761, t9713, t172, t2448, t763, t177, t2508);
        let t9722 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk814::<F>(t2512, t9490, t9720);
        let (t9724, t9725) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk815::<F>(t761, t9722, t9450, t9457, t9463, t9469, t9476, t9484, t9496, t9684, t9715, t9718);
    (t9715, t9716, t9718, t9720, t9722, t9724, t9725)
}
