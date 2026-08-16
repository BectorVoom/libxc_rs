//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1012/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1012(t12968: f64, t144: f64, t160: f64, t1643: f64, t1647: f64, t167: f64, t1901: f64, t2190: f64, t2205: f64, t379: f64, t38071: f64, t40594: f64, t41246: f64, t41251: f64, t41262: f64, t41264: f64, t41267: f64, t41269: f64, t446: f64, t558: f64, t569: f64, t604: f64, t616: f64, t7966: f64, t9017: f64, t9144: f64, t9316: f64) -> f64 {
    let t41278 = -8.0_f64 / 3.0_f64 * t446 * t569 * t616 * t7966 - 8.0_f64 / 3.0_f64 * t446 * t2205 * t167 * t38071 - 8.0_f64 / 3.0_f64 * t41246 + 8.0_f64 * t446 * t144 * t40594 + 8.0_f64 / 3.0_f64 * t1901 * t41251 * t160 * t9017 * t379 - 8.0_f64 * t1901 * t12968 * t604 * t558 * t9316 - 8.0_f64 / 9.0_f64 * t41262 - 16.0_f64 / 9.0_f64 * t41264 - 4.0_f64 / 9.0_f64 * t41267 - 8.0_f64 / 9.0_f64 * t1901 * t41269 * t1643 * t2190 + 8.0_f64 / 3.0_f64 * t1901 * t9144 * t1647 * t2190;
    t41278
}
