//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1550/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1550(t13042: f64, t24663: f64, t3172: f64, t12910: f64, t12916: f64, t24740: f64, t21143: f64, t5378: f64, t21192: f64, t5391: f64, t21107: f64, t5265: f64) -> (f64, f64, f64, f64, f64) {
    let t82469 = t13042 * t3172 * t24663;
    let t82491 = t12910 * t12916 * t24740;
    let t82534 = t21143 * t5378;
    let t82536 = t5391 * t21192;
    let t82550 = t21107 * t5265;
    (t82469, t82491, t82534, t82536, t82550)
}
