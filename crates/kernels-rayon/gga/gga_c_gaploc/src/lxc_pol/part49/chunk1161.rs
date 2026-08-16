//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1161/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1161(t13937: f64, t2549: f64, t12176: f64, t2558: f64, t943: f64, t1841: f64, t47484: f64, t7289: f64, t2576: f64, t39347: f64, t43166: f64, t43168: f64, t43170: f64, t47673: f64, t47677: f64, t47681: f64, t47685: f64) -> f64 {
    let t47687 = t2549 * t13937;
    let t47690 = t943 * t12176 * t2558;
    let t47693 = t1841 * t7289 * t47484;
    let t47696 = t1841 * t39347 * t2576;
    let t47699 = -0.76905262301422242837e-2_f64 * t47673 + 0.76905262301422242837e-2_f64 * t47677 + 0.92286314761706691403e-1_f64 * t47681 - 0.46143157380853345701e-1_f64 * t47685 + 0.32043859292259267849e-3_f64 * t47687 + 0.32043859292259267849e-3_f64 * t47690 - 0.17090058289204942852e-2_f64 * t47693 + 0.25635087433807414279e-2_f64 * t47696 - t43166 - t43168 + 0.76905262301422242837e-2_f64 * t43170;
    t47699
}
