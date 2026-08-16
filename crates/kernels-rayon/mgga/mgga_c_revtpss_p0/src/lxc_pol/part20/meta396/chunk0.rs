//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1457/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1457(t11289: f64, t2919: f64, t2866: f64, t2923: f64, t2927: f64, t11380: f64, t2869: f64, t11384: f64, t910: f64, t11388: f64, t275: f64, t2872: f64, t2922: f64) -> (f64, f64, f64, f64, f64) {
    let t41577 = 6.0_f64 * t11289 * t2919;
    let t41578 = t2866 * t2923;
    let t41580 = 0.96491876992155210402e2_f64 * t41578 * t2927;
    let t41582 = 4.0_f64 * t2869 * t11380;
    let t41583 = t910 * t11384;
    let t41585 = 0.2069040516770936012e4_f64 * t41583 * t11388;
    let t41588 = t275 / t2922 / t2872;
    (t41577, t41580, t41582, t41585, t41588)
}
