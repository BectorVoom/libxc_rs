//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 640/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk640(t422: f64, t5239: f64, t5238: f64, t5236: f64, t2335: f64, t496: f64, t190: f64, t430: f64, t1006: f64, t1567: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5240 = t5239 * t422;
    let t5241 = 1.0_f64 / t5240;
    let t5242 = t5238 * t5241;
    let t5243 = t5236 * t5242;
    let t5245 = t2335 * t496;
    let t5246 = t430 * t190 * t5245;
    let t5249 = t1006 * t1567;
    (t5241, t5242, t5243, t5245, t5246, t5249)
}
