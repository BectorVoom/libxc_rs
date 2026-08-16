//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1349/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1349(t2612: f64, t40207: f64, t190: f64, t2611: f64, t39449: f64, t40076: f64, t40079: f64, t40184: f64, t40187: f64, t40190: f64, t40194: f64, t40198: f64, t40202: f64, t40204: f64, t40206: f64) -> (f64, f64, f64) {
    let t40209 = 72.0_f64 * t40207 * t2612;
    let t40212 = 36.0_f64 * t2611 * t190 * t39449;
    let t40213 = -t40184 + t40187 + t40190 + t40076 - t40079 + t40194 + t40198 + t40202 + t40204 - t40206 + t40209 + t40212;
    (t40209, t40212, t40213)
}
