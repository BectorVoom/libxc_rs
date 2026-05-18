//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1343/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1343<F: Float>(t43347: F, t52269: F, t57327: F, t57330: F, t57332: F, t57335: F, t57337: F, t57343: F, t57346: F, t57349: F, t57351: F, t57527: F) -> F {
    let t58240 = F::new(2.0) / F::new(9.0) * t52269 - t57327 + F::new(4.0) / F::new(9.0) * t43347 - t57330 - t57332 - t57335 + t57337 - t57343 + t57346 + t57349 - t57351 - t57527;
    t58240
}
