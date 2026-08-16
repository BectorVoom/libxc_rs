//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1040/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1040(t86354: f64, t86370: f64, t86386: f64, t86402: f64, t16246: f64, t4589: f64, t103: f64, t11902: f64, t11906: f64, t15901: f64, t16030: f64, t1901: f64, t20225: f64, t20229: f64, t20287: f64, t28: f64, t39317: f64, t446: f64, t4611: f64, t47926: f64, t60984: f64, t75766: f64, t75845: f64, t82: f64, t83: f64, t8557: f64, t89: f64) -> (f64, f64, f64) {
    let t86404 = t86354 + t86370 + t86386 + t86402;
    let t86411 = t16246 * t4589;
    let t86422 = -4.0_f64 / 3.0_f64 * t1901 * t8557 * t15901 * t4611 - 8.0_f64 / 3.0_f64 * t75766 - 16.0_f64 / 27.0_f64 * t60984 + 4.0_f64 / 3.0_f64 * t1901 * t11902 * t20287 + t89 * t28 * t82 * t86404 * t103 / 3.0_f64 + 112.0_f64 / 243.0_f64 * t47926 + t39317 - 2.0_f64 * t446 * t83 * t86411 - 4.0_f64 / 9.0_f64 * t75845 + 8.0_f64 / 3.0_f64 * t1901 * t11906 * t20225 - 8.0_f64 / 9.0_f64 * t1901 * t16030 * t20229;
    (t86404, t86411, t86422)
}
