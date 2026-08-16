//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 939/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk939(t3720: f64, t5241: f64, t2679: f64, t9805: f64, t2610: f64, t38907: f64, t2033: f64, t2365: f64, t39040: f64, t6111: f64, t12251: f64, t2021: f64, t7372: f64) -> (f64, f64, f64, f64, f64) {
    let t47168 = t5241 * t3720;
    let t47170 = t9805 * t47168 * t2679;
    let t47178 = t2610 * t38907;
    let t47180 = t2033 * t2365 * t47178;
    let t47196 = t6111 * t2365 * t39040;
    let t47199 = t2021 * t12251 * t7372;
    (t47170, t47178, t47180, t47196, t47199)
}
