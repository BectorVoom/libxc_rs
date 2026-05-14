//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1263/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1263<F: Float>(t121914: F, t24432: F, t6118: F, t24448: F, t30991: F, t681: F, t193: F, t30859: F, t6109: F, t743: F, t747: F, t122609: F, t1434: F, t2506: F, t124029: F, t446: F, t9770: F) -> (F, F, F, F, F) {
    let t124232 = t6118 * t24432 * t121914;
    let t124235 = t24448 * t681 * t30991;
    let t124240 = t6109 * t193 * t743 * t30859 * t747;
    let t124244 = t1434 * t193 * t2506 * t122609;
    let t124247 = t446 * t9770 * t124029;
    (t124232, t124235, t124240, t124244, t124247)
}
