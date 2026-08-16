//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 878/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk878(t645: f64, t7582: f64, t7524: f64, t7526: f64, t7529: f64, t7532: f64, t7536: f64, t7538: f64, t7540: f64, t7541: f64, t7563: f64, t7567: f64, t7569: f64, t7572: f64, t7573: f64, t7576: f64, t7578: f64, t7581: f64) -> (f64, f64) {
    let t7584 = 8.0_f64 / 45.0_f64 * t7582 * t645;
    let t7585 = t7524 + t7526 - t7529 - t7532 - t7536 - t7538 + t7540 - 2.0_f64 / 27.0_f64 * t7541 + t7563 - t7567 - t7569 - t7572 + 0.33245444444444444444e-1_f64 * t7573 - t7576 + t7578 - t7581 + t7584;
    (t7584, t7585)
}
