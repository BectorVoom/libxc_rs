//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 975/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk975(t670: f64, t8342: f64, t117: f64, t8320: f64, t1459: f64, t1461: f64, t2207: f64, t2209: f64, t572: f64, t573: f64, t8336: f64, t1843: f64, t2198: f64) -> (f64, f64, f64, f64) {
    let t8343 = t8342 * t670;
    let t8346 = t117 * t8320;
    let t8349 = 3.0_f64 * t1459 * t2209 + 3.0_f64 * t1461 * t2207 + 6.0_f64 * t572 * t8343 + 3.0_f64 * t572 * t8346 + t573 * t8336;
    let t8393 = t1843 * t2198;
    (t8343, t8346, t8349, t8393)
}
