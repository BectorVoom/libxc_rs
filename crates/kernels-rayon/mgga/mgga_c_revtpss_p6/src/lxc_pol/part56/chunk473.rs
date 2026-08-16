//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 473/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk473(t3140: f64, t3143: f64, t342: f64, t335: f64, t368: f64, t1035: f64, t389: f64, t1941: f64, t268: f64, t404: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3298 = t3140 * t3143;
    let t3299 = t342 * t3298;
    let t3302 = 1.0_f64 / t368 / t335;
    let t3316 = t3140 * t1035;
    let t3317 = t342 * t3316;
    let t3335 = t389 * t389;
    let t3336 = 1.0_f64 / t3335;
    let t3356 = t268 * t1941 * t404;
    (t3298, t3299, t3302, t3316, t3317, t3336, t3356)
}
