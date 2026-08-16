//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 653/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk653(t10: f64, t144: f64, t3050: f64, t1984: f64, t378: f64, t1986: f64, t379: f64, t446: f64, t1647: f64, t558: f64, t1969: f64, t9039: f64, t9043: f64, t9047: f64, t9052: f64, t9057: f64, t9059: f64, t9062: f64, t9065: f64, t9068: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9071 = t10 * t3050 * t144;
    let t9072 = 14.0_f64 / 81.0_f64 * t9071;
    let t9073 = t378 * t1984;
    let t9074 = t379 * t1986;
    let t9075 = t9073 * t9074;
    let t9076 = t446 * t9075;
    let t9078 = t1647 * t558;
    let t9079 = t1969 * t9078;
    let t9080 = t446 * t9079;
    let t9082 = -t9039 / 9.0_f64 + t9043 / 6.0_f64 + t9047 / 6.0_f64 + t9052 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t9057 - t9059 / 9.0_f64 - t9062 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t9065 + t9068 / 6.0_f64 - t9072 - t9076 / 3.0_f64 - t9080 / 3.0_f64;
    (t9071, t9073, t9074, t9075, t9076, t9078, t9079, t9080, t9082)
}
