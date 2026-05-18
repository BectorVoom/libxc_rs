//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 950/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk950<F: Float>(t40261: F, t40372: F, t47877: F, t587: F, t912: F, t1: F, t47008: F, t1415: F, t13778: F, t589: F, t40449: F, t40452: F) -> (F, F, F, F, F, F, F, F) {
    let t47994 = F::new(0.12780975317973583226e0) * t40261;
    let t48070 = F::new(0.63904876589867916128e-1) * t40372;
    let t48081 = t587 * t912 * t47877;
    let t48086 = t47008 * t1;
    let t48087 = t1415 * t48086;
    let t48121 = t587 * t589 * t13778;
    let t48140 = F::new(0.63904876589867916128e-1) * t40449;
    let t48141 = F::new(0.31952438294933958064e0) * t40452;
    (t47994, t48070, t48081, t48086, t48087, t48121, t48140, t48141)
}
