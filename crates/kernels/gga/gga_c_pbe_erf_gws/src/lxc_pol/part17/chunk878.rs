//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 878/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk878<F: Float>(t645: F, t7582: F, t7524: F, t7526: F, t7529: F, t7532: F, t7536: F, t7538: F, t7540: F, t7541: F, t7563: F, t7567: F, t7569: F, t7572: F, t7573: F, t7576: F, t7578: F, t7581: F) -> (F, F) {
    let t7584 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t7582 * t645;
    let t7585 = t7524 + t7526 - t7529 - t7532 - t7536 - t7538 + t7540 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t7541 + t7563 - t7567 - t7569 - t7572 + F::cast_from(0.33245444444444444444e-1_f64) * t7573 - t7576 + t7578 - t7581 + t7584;
    (t7584, t7585)
}
