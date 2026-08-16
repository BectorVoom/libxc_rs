//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 678/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk678(t31: f64, t3648: f64, t4: f64, t14: f64, t2: f64, t25: f64, t39: f64, t1765: f64, t745: f64, t1764: f64, t518: f64, t622: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6359 = 0.34451131037037037036e-2_f64 * t4 * t3648 * t31;
    let t6363 = 1.0_f64 / t14 / t25 * t2 / 4.0_f64;
    let t6364 = t6363 * t39;
    let t6366 = t1765 * t745;
    let t6367 = t1764 * t6366;
    let t6369 = t518 * t622;
    (t6359, t6363, t6364, t6366, t6367, t6369)
}
