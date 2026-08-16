//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2471/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2471<F: Float>(t1070: F, t193: F, t336: F, t47793: F, t47795: F, t47798: F, t47802: F, t48679: F, t48681: F, t48725: F, t48727: F, t48730: F, t48732: F, t50648: F, t50678: F, t50712: F, t50744: F) -> F {
    let t50750 = t47793 - t47795 + t47798 + t47802 + t193 * t336 * (t50648 + t50678 + t50712 + t50744) * t1070 - t48679 - t48681 - t48725 - t48727 - t48730 - t48732;
    t50750
}
