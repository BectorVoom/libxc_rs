//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1102/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1102(t3300: f64, t39066: f64, t1980: f64, t7458: f64, t1846: f64, t7712: f64, t1988: f64, t9724: f64, t2001: f64, t5966: f64, t1851: f64, t7605: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39271 = t3300 * t39066;
    let t39273 = t1980 * t7458 * t39271;
    let t39275 = t7712 * t1846;
    let t39277 = t1988 * t9724;
    let t39279 = t2001 * t5966;
    let t39281 = t7605 * t1851;
    (t39271, t39273, t39275, t39277, t39279, t39281)
}
