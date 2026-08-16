//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1125/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1125(t16438: f64, t654: f64, t127: f64, t16370: f64, t16394: f64, t2030: f64, t16382: f64, t16406: f64, t16402: f64, t16386: f64, t6799: f64, t16323: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48272 = t654 * t16438;
    let t48308 = t16370 * t127;
    let t48313 = t2030 * t16394;
    let t48315 = t2030 * t16382;
    let t48317 = t2030 * t16406;
    let t48320 = t2030 * t16402;
    let t48356 = t6799 * t16386;
    let t48365 = t16323 * t127;
    (t48272, t48308, t48313, t48315, t48317, t48320, t48356, t48365)
}
