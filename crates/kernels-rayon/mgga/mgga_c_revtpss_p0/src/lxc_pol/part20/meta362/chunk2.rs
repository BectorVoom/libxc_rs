//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1317/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1317(t10587: f64, t2516: f64, t157: f64, t190: f64, t39443: f64, t2401: f64, t2609: f64, t2519: f64, t268: f64, t9306: f64) -> (f64, f64, f64, f64) {
    let t39774 = t10587 * t2516;
    let t39775 = 0.35089341735807877242e1_f64 * t39774;
    let t39778 = 24.0_f64 * t39443 * t157 * t190;
    let t39779 = t2401 * t2609;
    let t39780 = 6.0_f64 * t39779;
    let t39783 = 0.71233333333333333332e-1_f64 * t268 * t2519 * t9306;
    (t39775, t39778, t39780, t39783)
}
