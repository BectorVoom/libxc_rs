//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1046/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1046(t2629: f64, t9866: f64, t9575: f64, t9572: f64, t177: f64, t2390: f64, t762: f64, t760: f64, t9419: f64, t2516: f64, t2523: f64, t9387: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10582 = 0.48159733137676571078e0_f64 * t2629 * t9866;
    let t10584 = 0.21687162600603479684e-1_f64 * t2629 * t9575;
    let t10586 = 0.32530743900905219526e-1_f64 * t2629 * t9572;
    let t10587 = t2390 * t177;
    let t10588 = t10587 * t762;
    let t10592 = 0.10389515463408878255e3_f64 * t760 * t9419;
    let t10593 = t2523 * t2516;
    let t10596 = 0.5848223622634646207e0_f64 * t760 * t9387;
    (t10582, t10584, t10586, t10588, t10592, t10593, t10596)
}
