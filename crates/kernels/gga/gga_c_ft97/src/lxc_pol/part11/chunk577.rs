//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 577/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk577<F: Float>(t1906: F, t8392: F, t1922: F, t432: F, t452: F, t1755: F, t499: F, t110: F, t8183: F, t447: F, t7966: F, t1873: F, t1882: F, t24: F, t7241: F, t7751: F) -> (F, F, F, F, F, F, F, F) {
    let t8393 = t8392 * t1906;
    let t8396 = t452 * t1922 * t432;
    let t8399 = t452 * t499 * t1755;
    let t8402 = t452 * t110 * t8183;
    let t8406 = t447 * t110 * t7966;
    let t8409 = t1882 * t1873;
    let t8411 = t24 * t7241;
    let t8413 = t8411 * t110 * t7751;
    (t8393, t8396, t8399, t8402, t8406, t8409, t8411, t8413)
}
