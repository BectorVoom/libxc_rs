//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 609/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk609<F: Float>(t3451: F, t738: F, t270: F, t3242: F, t3250: F, t3422: F, t3434: F, t3439: F, t3442: F, t3446: F, t3450: F, t2969: F, t977: F) -> (F, F, F) {
    let t3452 = t738 * t3451;
    let t3455 = t3422 + F::new(0.76905262301422242837e-2) * t270 * t3434 + t3439 - t3442 + t3242 - t3250 - t3446 + t3450 - F::new(0.76905262301422242837e-2) * t270 * t3452;
    let t3457 = t2969 * t977;
    (t3452, t3455, t3457)
}
