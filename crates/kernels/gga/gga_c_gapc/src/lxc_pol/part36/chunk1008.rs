//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1008/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1008<F: Float>(t12328: F, t12330: F, t12332: F, t12334: F, t12336: F, t12338: F, t12341: F, t12344: F, t12345: F, t12348: F, t12434: F, t12572: F) -> F {
    let t12573 = -t12328 + t12330 + t12332 - t12334 + t12336 - t12338 + t12341 - t12344 + t12345 - t12348 + t12434;
    let t12574 = t12572 + t12573;
    t12574
}
