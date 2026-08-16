//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1168/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1168(t6193: f64, t8217: f64, t2260: f64, t27668: f64, t28465: f64, t28508: f64, t28547: f64, t28727: f64, t28856: f64, t29366: f64, t29370: f64, t29373: f64, t29381: f64, t29384: f64, t29387: f64, t29397: f64, t29402: f64, t8213: f64) -> (f64, f64) {
    let t29608 = t6193 * t8217;
    let t29622 = -0.61905925925925925925e-2_f64 * t29366 - 0.23214722222222222222e-2_f64 * t29370 + 0.17411041666666666666e-2_f64 * t29373 + 0.18534722222222222222e-2_f64 * t29608 * t2260 - 0.18534722222222222222e-2_f64 * t28727 * t8213 + 0.23214722222222222222e-2_f64 * t28465 + 0.30918233506944444444e-4_f64 * t28856 - 0.61905925925925925925e-2_f64 * t28508 - 0.17411041666666666666e-2_f64 * t29381 + 0.34822083333333333332e-2_f64 * t29384 + 0.92858888888888888886e-2_f64 * t29387 - t27668 - 0.23214722222222222222e-2_f64 * t28547 - 0.92858888888888888886e-2_f64 * t29397 + 0.17024129629629629629e-1_f64 * t29402;
    (t29608, t29622)
}
