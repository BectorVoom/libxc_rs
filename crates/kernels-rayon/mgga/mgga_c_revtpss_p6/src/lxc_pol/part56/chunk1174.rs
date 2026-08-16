//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1174/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1174(t124533: f64, t125344: f64, t129478: f64, t129479: f64, t129480: f64, t129481: f64, t129482: f64, t129483: f64, t129488: f64, t131234: f64, t131338: f64, t1518: f64, t33346: f64, t33644: f64, t33646: f64, t4292: f64, t670: f64) -> f64 {
    let t131384 = 2.0_f64 * t124533 * t1518 + 2.0_f64 * t131234 * t670 + 2.0_f64 * t131338 * t1518 + 2.0_f64 * t33346 * t4292 + t125344 + 4.0_f64 * t129478 + 4.0_f64 * t129479 + 4.0_f64 * t129480 + 4.0_f64 * t129481 + 4.0_f64 * t129482 + 4.0_f64 * t129483 + 4.0_f64 * t129488 + t33644 + t33646;
    t131384
}
