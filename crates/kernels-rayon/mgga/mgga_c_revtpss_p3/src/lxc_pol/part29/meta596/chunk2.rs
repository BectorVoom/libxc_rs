//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2005/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2005(t28447: f64, t689: f64, t887: f64, t26485: f64, t99463: f64, t102986: f64, t25387: f64, t1580: f64, t2439: f64, t26434: f64, t2453: f64, t2458: f64, t7998: f64) -> (f64, f64, f64, f64, f64) {
    let t103140 = 0.10975748638225852664e-1_f64 * t689 * t28447 * t887;
    let t103142 = 0.51405703062096148812e-1_f64 * t99463 * t26485;
    let t103156 = 0.51405703062096148812e-1_f64 * t25387 * t102986;
    let t103158 = t2439 * t26434 * t1580;
    let t103161 = t2453 * t7998 * t2458;
    (t103140, t103142, t103156, t103158, t103161)
}
