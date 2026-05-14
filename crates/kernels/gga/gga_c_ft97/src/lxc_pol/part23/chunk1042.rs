//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1042/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1042<F: Float>(t2781: F, t31613: F, t1486: F, t193: F, t1234: F, t7021: F, t852: F, t6308: F, t31551: F, t799: F, t27: F, t89: F, t1476: F, t5299: F) -> (F, F, F, F, F, F, F, F) {
    let t31614 = t2781 * t31613;
    let t31616 = t1486 * t193 * t31614;
    let t31618 = t7021 * t1234;
    let t31619 = t852 * t31618;
    let t31621 = t6308 * t193 * t31619;
    let t31624 = t799 * t31551;
    let t31626 = t89 * t27 * t31624;
    let t31627 = t1476 * t5299;
    (t31614, t31616, t31618, t31619, t31621, t31624, t31626, t31627)
}
