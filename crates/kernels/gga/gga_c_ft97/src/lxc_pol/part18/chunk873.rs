//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 873/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk873<F: Float>(t2190: F, t23455: F, t13140: F, t5943: F, t9099: F, t5968: F, t604: F, t379: F, t2210: F, t5944: F, t8392: F, t1378: F, t582: F) -> (F, F, F, F, F, F, F, F) {
    let t23456 = t23455 * t2190;
    let t23457 = t13140 * t23456;
    let t23460 = t9099 * t5943;
    let t23463 = t604 * t5968;
    let t23464 = t23463 * t379;
    let t23465 = t2210 * t23464;
    let t23468 = t8392 * t5944;
    let t23470 = t582 * t1378;
    (t23456, t23457, t23460, t23463, t23464, t23465, t23468, t23470)
}
