//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1484/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1484(t31303: f64, t31326: f64, t3: f64, t2178: f64, t670: f64, t1518: f64, t31117: f64, t4292: f64, t8295: f64, t116: f64, t8362: f64, t117: f64, t31292: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31328 = 2.0_f64 * t31303 + 2.0_f64 * t31326;
    let t31329 = t3 * t31328;
    let t31340 = param_d * t31328;
    let t31358 = t670 * t2178;
    let t31359 = t31358 * t1518;
    let t31362 = t31117 * t1518;
    let t31365 = t8295 * t4292;
    let t31370 = t116 * t8362;
    let t31371 = t31370 * t670;
    let t31374 = t117 * t31292;
    (t31328, t31329, t31340, t31358, t31359, t31362, t31365, t31370, t31371, t31374)
}
