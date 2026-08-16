//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 862/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk862(t12792: f64, t1564: f64, t12887: f64, t1641: f64, t39624: f64, t39626: f64, t39632: f64, t39637: f64, t39642: f64, t39646: f64, t39648: f64, t39650: f64, t471: f64) -> (f64, f64, f64) {
    let t42093 = t1564 * t12792;
    let t42099 = 0.92023022289409799224e1_f64 * t1641 * t12887;
    let t42111 = (21.0_f64 / 512.0_f64 * t39624 + 357.0_f64 / 16384.0_f64 * t39626 - 189.0_f64 / 262144.0_f64 * t39632 + 189.0_f64 / 0.16777216e8_f64 * t39637 - 63.0_f64 / 0.16777216e8_f64 * t39642 + 63.0_f64 / 262144.0_f64 * t39646 - 119.0_f64 / 16384.0_f64 * t39648 - 7.0_f64 / 512.0_f64 * t39650) * t471;
    (t42093, t42099, t42111)
}
