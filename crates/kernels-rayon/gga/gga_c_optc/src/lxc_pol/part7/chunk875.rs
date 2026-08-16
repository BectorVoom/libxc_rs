//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 875/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk875(t43: f64, t7249: f64, t7321: f64, t8303: f64, t8397: f64, t6541: f64, t176: f64, t2902: f64, t1219: f64, t2848: f64, t50: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t8399 = t7249 + t7321 + t8303 + t8397;
    let t8406 = piecewise3(t44, 0.0_f64, t6541);
    let t8409 = t176 * t2902;
    let t8410 = t8409 * t1219;
    let t8414 = 1.0_f64 / t2848 / t50;
    (t8399, t8406, t8410, t8414)
}
