//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 858/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk858<F: Float>(t224: F, t44670: F, t44671: F, t44674: F, t44676: F, t44678: F, t44679: F, t44681: F, t44684: F, t44687: F, t44689: F, t44692: F, t44694: F, t44704: F, t44705: F, t44706: F, t45123: F, t45126: F, t45130: F, t45132: F, t45161: F, t45994: F, t46836: F) -> (F,) {
    let t46840 = t44670 - t44671 - t44674 + t44676 - t44678 - t44679 + t44681 - t44684 + t44687 - t44689 + t44692 - t44694 + t224 * (t44706 + t45161 + t45994 + t46836) - t44704 - t44705 + t45123 - t45126 + t45130 - t45132;
    (t46840,)
}
