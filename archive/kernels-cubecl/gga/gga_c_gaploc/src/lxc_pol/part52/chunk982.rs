//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 982/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk982<F: Float>(t1052: F, t14408: F, t1955: F, t38885: F, t45124: F, t45134: F, t45141: F, t45144: F, t45146: F, t45148: F, t45151: F, t45163: F, t45164: F, t45967: F, t45969: F, t45973: F, t49970: F, t49972: F, t49974: F, t49977: F) -> F {
    let t50312 = -F::cast_from(2.0_f64) * t1052 * t38885 - t14408 * t1955 - t45124 - t45134 + t45141 + t45144 - t45146 - t45148 + t45151 - t45163 + t45164 - t45967 - t45969 - t45973 + t49970 - t49972 + t49974 + t49977;
    t50312
}
