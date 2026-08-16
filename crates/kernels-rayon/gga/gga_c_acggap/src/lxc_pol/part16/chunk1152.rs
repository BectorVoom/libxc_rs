//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1152/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1152(t1181: f64, t5572: f64, t7351: f64, t7575: f64, t2016: f64, t9618: f64, t1488: f64, t2030: f64, t2313: f64, t2001: f64, t5551: f64, t1856: f64, t7605: f64) -> (f64, f64, f64, f64, f64) {
    let t39923 = t7575 * t1181 * t7351 * t5572;
    let t39925 = t2016 * t9618;
    let t39928 = t2030 * t1488 * t2313;
    let t39930 = t2001 * t5551;
    let t39932 = t7605 * t1856;
    (t39923, t39925, t39928, t39930, t39932)
}
