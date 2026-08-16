//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1101/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1101(t10852: f64, t2253: f64, t170: f64, t328: f64, t39600: f64, t10850: f64, t10904: f64, t10915: f64, t14514: f64, t14519: f64, t2265: f64, t231: f64, t2928: f64, t2938: f64, t2939: f64, t2951: f64, t41448: f64, t41468: f64, t43046: f64, t43050: f64, t43062: f64, t43072: f64, t43074: f64, t43076: f64, t43078: f64, t631: f64, t898: f64) -> f64 {
    let t43080 = t2253 * t10852;
    let t43084 = 220.0_f64 / 81.0_f64 * t170 * t39600 * t328;
    let t43088 = 8.0_f64 * t2265 * t14519 * t43046 - 8.0_f64 / 9.0_f64 * t631 * t10915 * t43050 * t41448 - 4.0_f64 * t631 * t231 * t10850 * t41448 - t631 * t231 * t2928 * t41468 - 9.0_f64 / 2.0_f64 * t631 * t898 * t2938 * t43062 + 36.0_f64 * t631 * t898 * t10904 * t2939 * t2951 - 20.0_f64 / 9.0_f64 * t43072 - 8.0_f64 / 3.0_f64 * t43074 - 16.0_f64 / 81.0_f64 * t43076 - 4.0_f64 / 9.0_f64 * t43078 + 8.0_f64 / 9.0_f64 * t43080 - t43084 - 4.0_f64 / 3.0_f64 * t2265 * t14514 * t43046;
    t43088
}
