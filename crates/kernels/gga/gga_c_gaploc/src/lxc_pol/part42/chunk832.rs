//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 832/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk832<F: Float>(t44671: F, t44674: F, t44676: F, t44678: F, t44684: F, t44687: F, t44689: F, t44692: F, t44694: F, t44705: F, t45134: F, t45148: F, t45151: F, t49820: F, t49965: F, t49968: F, t49970: F, t49972: F, t49974: F) -> (F,) {
    let t49975 = -t44671 - t44674 - t49820 + t44676 - t44678 - t44684 + t44687 - t44689 + t44692 - t44694 - t44705 + t49965 + t49968 - t49970 + t45134 + t45148 - t45151 + t49972 - t49974;
    (t49975,)
}
