//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2368/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2368(t40196: f64, t760: f64, t10587: f64, t2626: f64, t2523: f64, t9425: f64, t2389: f64, t37: f64, t2394: f64, t2475: f64, t10069: f64, t10929: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40198 = 0.35089341735807877242e1_f64 * t760 * t40196;
    let t40203 = t10587 * t2626;
    let t40205 = t2523 * t9425;
    let t40207 = t37 * t2389;
    let t40236 = t2475 * t2394;
    let t40267 = t10069 * t10929;
    (t40198, t40203, t40205, t40207, t40236, t40267)
}
