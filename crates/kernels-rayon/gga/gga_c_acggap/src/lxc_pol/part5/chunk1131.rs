//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1131/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1131(t1008: f64, t6110: f64, t1005: f64, t5971: f64, t1089: f64, t175: f64, t322: f64, t384: f64, t5506: f64, t1734: f64, t879: f64, t5826: f64) -> (f64, f64, f64, f64, f64) {
    let t20238 = t1008 * t6110;
    let t20263 = t1005 * t5971;
    let t20268 = t384 * t1089 * t175 * t5506 * t322;
    let t20273 = t384 * t1089 * t175 * t1734 * t879;
    let t20275 = t1005 * t5826;
    (t20238, t20263, t20268, t20273, t20275)
}
