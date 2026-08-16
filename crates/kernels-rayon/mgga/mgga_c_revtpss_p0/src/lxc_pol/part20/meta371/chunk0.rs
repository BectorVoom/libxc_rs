//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1348/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1348(t40196: f64, t760: f64, t14330: f64, t189: f64, t2251: f64, t2258: f64, t10587: f64, t2626: f64, t2523: f64, t9425: f64, t2389: f64, t37: f64) -> (f64, f64, f64, f64, f64) {
    let t40198 = 0.35089341735807877242e1_f64 * t760 * t40196;
    let t40202 = 144.0_f64 * t14330 * t189 * t2251 * t2258;
    let t40203 = t10587 * t2626;
    let t40204 = 0.70178683471615754484e1_f64 * t40203;
    let t40205 = t2523 * t9425;
    let t40206 = 0.14035736694323150897e2_f64 * t40205;
    let t40207 = t37 * t2389;
    (t40198, t40202, t40204, t40206, t40207)
}
