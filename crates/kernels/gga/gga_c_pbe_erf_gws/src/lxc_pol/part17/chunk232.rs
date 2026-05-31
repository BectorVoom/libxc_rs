//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 232/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk232<F: Float>(t242: F, t528: F, t3: F, t551: F, t156: F, t161: F, t546: F) -> (F, F, F, F) {
    let t692 = F::cast_from(0.83762820535504401876e-1_f64) * t528 * t242;
    let t696 = t551 * t3;
    let t697 = t156 * t161;
    let t700 = t546 / F::cast_from(2.0_f64) + F::cast_from(0.3135e-1_f64) * t696 * t697;
    (t692, t696, t697, t700)
}
