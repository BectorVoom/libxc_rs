//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1207/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1207(t123: f64, t752: f64, t2630: f64, t2629: f64, t9866: f64, t9575: f64, t9572: f64, t760: f64, t9419: f64, t2516: f64, t2523: f64, t9387: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10578 = t752 * t123;
    let t10579 = t10578 * t2630;
    let t10582 = 0.48159733137676571078e0_f64 * t2629 * t9866;
    let t10584 = 0.21687162600603479684e-1_f64 * t2629 * t9575;
    let t10586 = 0.32530743900905219526e-1_f64 * t2629 * t9572;
    let t10592 = 0.10389515463408878255e3_f64 * t760 * t9419;
    let t10593 = t2523 * t2516;
    let t10596 = 0.5848223622634646207e0_f64 * t760 * t9387;
    (t10579, t10582, t10584, t10586, t10592, t10593, t10596)
}
