//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1448/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1448(t41291: f64, t41389: f64, t41421: f64, t41443: f64, t964: f64, t973: f64, t981: f64, t11591: f64, t3026: f64, t3034: f64, t3030: f64, t11465: f64, t41225: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41445 = t41291 + t41389 + t41421 + t41443;
    let t41449 = 0.5848223622634646207e0_f64 * t981 * t964 * t41445 * t973;
    let t41451 = 0.70178683471615754484e1_f64 * t11591 * t3026;
    let t41453 = 0.10389515463408878255e3_f64 * t11591 * t3034;
    let t41455 = 0.35089341735807877242e1_f64 * t11591 * t3030;
    let t41459 = 0.14035736694323150897e2_f64 * t981 * t11465 * t41225 * t973;
    (t41445, t41449, t41451, t41453, t41455, t41459)
}
