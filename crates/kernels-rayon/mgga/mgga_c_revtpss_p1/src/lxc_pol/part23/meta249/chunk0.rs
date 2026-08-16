//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1431/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1431(t730: f64, t9434: f64, t2552: f64, t722: f64, t164: f64, t172: f64, t2555: f64, t177: f64, t9367: f64, t9368: f64, t9371: f64, t701: f64, t9275: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9525 = t9434 * t730;
    let t9529 = 1.0_f64 / t2552 / t722;
    let t9530 = t164 * t9529;
    let t9532 = 1.0_f64 / t2555 / t172;
    let t9533 = t9434 * t9532;
    let t9536 = t177 * t9367;
    let t9537 = t9368 * t9371;
    let t9540 = t9275 * t701;
    (t9525, t9529, t9530, t9532, t9533, t9536, t9537, t9540)
}
