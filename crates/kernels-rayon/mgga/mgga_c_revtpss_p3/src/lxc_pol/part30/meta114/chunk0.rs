//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 672/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk672(t680: f64, t130: f64, t146: f64, t2566: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2580 = t680 * t680;
    let t2581 = 1.0_f64 / t2580;
    let t2582 = t130 * t2581;
    let t2583 = t146 * t146;
    let t2584 = 1.0_f64 / t2583;
    let t2585 = t2566 * t2584;
    let t2587 = 0.16081979498692535067e2_f64 * t2582 * t2585;
    (t2580, t2581, t2582, t2583, t2584, t2585, t2587)
}
