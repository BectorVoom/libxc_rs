//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2913/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2913(t11606: f64, t4719: f64, t1642: f64, t41491: f64, t11591: f64, t4729: f64, t52229: f64, t52231: f64, t52235: f64, t52237: f64, t52242: f64, t52245: f64, t52860: f64, t52863: f64) -> (f64, f64, f64, f64) {
    let t52865 = 0.10389515463408878255e3_f64 * t4719 * t11606;
    let t52867 = 0.5848223622634646207e0_f64 * t41491 * t1642;
    let t52869 = 0.17544670867903938621e1_f64 * t11591 * t4729;
    let t52870 = t52229 + t52231 + t52235 + t52237 + t52242 + t52245 + t52860 + t52863 + t52865 - t52867 - t52869;
    (t52865, t52867, t52869, t52870)
}
