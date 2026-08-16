//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 608/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk608(t3451: f64, t738: f64, t270: f64, t3242: f64, t3250: f64, t3422: f64, t3434: f64, t3439: f64, t3442: f64, t3446: f64, t3450: f64, t2969: f64, t977: f64) -> (f64, f64, f64) {
    let t3452 = t738 * t3451;
    let t3455 = t3422 + 0.76905262301422242837e-2_f64 * t270 * t3434 + t3439 - t3442 + t3242 - t3250 - t3446 + t3450 - 0.76905262301422242837e-2_f64 * t270 * t3452;
    let t3457 = t2969 * t977;
    (t3452, t3455, t3457)
}
