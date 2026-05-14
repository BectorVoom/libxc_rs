//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1167/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1167<F: Float>(t39565: F, t49404: F, t49406: F, t57057: F, t57060: F, t57063: F, t57066: F, t57069: F, t57071: F, t57073: F, t57100: F, t57102: F, t57104: F, t57106: F, t57135: F, t57148: F, t57164: F) -> (F,) {
    let t57179 = 0.11038e1 * t39565 + 0.132456e1 * t49404 - 0.44152e0 * t49406 - 0.301925e0 * t57057 + 0.72462e1 * t57060 + 0.181155e1 * t57063 + 0.247573125e0 * t57066 + 0.6189328125e-1 * t57069 - 0.3883875e1 * t57071 - 0.485484375e1 * t57073 + 0.16504875e0 * t57100 + 0.11651625e2 * t57102 - 0.51785e1 * t57104 + 0.258925e1 * t57106;
    let t57181 = t57135 + t57148 + t57164 + t57179;
    (t57181,)
}
