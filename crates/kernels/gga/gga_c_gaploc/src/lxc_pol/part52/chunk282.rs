//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 282/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk282<F: Float>(t501: F, t892: F, t605: F, t921: F, t589: F, t913: F, t587: F, t123: F, t160: F, t90: F) -> (F, F, F, F) {
    let t2355 = t892 * t501;
    let t2358 = t921 * t605;
    let t2361 = t589 * t913;
    let t2362 = t587 * t2361;
    let t2365 = t90 * t123 * t160;
    (t2355, t2358, t2362, t2365)
}
