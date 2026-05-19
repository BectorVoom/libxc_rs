//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 617/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk617<F: Float>(t270: F, t3242: F, t3250: F, t3422: F, t3439: F, t3442: F, t3446: F, t3450: F, t3723: F, t3727: F, t314: F, t3720: F) -> (F, F) {
    let t3730 = t3422 + F::cast_from(0.76905262301422242837e-2_f64) * t270 * t3723 + t3242 - t3442 + t3439 - t3446 - t3250 + t3450 - F::cast_from(0.76905262301422242837e-2_f64) * t270 * t3727;
    let t3732 = t314 * t3720;
    (t3730, t3732)
}
