//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1622/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1622(t50888: f64, t62300: f64, t50892: f64, t50893: f64, t77047: f64, t50901: f64, t40076: f64, t40079: f64, t40184: f64, t40194: f64, t40198: f64, t87673: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t87674 = 0.14035736694323150897e2_f64 * t50888;
    let t87675 = 6.0_f64 * t62300;
    let t87676 = 4.0_f64 * t50892;
    let t87677 = 0.4155806185363551302e3_f64 * t50893;
    let t87678 = 0.23392894490538584828e1_f64 * t77047;
    let t87679 = 0.1301229756036208781e0_f64 * t50901;
    let t87680 = -t40184 + t87673 - t87674 + t87675 + t87676 + t87677 - t87678 + t40076 - t40079 + t40194 + t40198 - t87679;
    (t87674, t87675, t87676, t87677, t87678, t87679, t87680)
}
