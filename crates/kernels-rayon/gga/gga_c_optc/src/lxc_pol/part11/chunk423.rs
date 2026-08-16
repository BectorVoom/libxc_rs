//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 423/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk423(t228: f64, t777: f64, t216: f64, t214: f64, t217: f64, t2257: f64, t2280: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2371 = t777 * t228;
    let t2372 = 1.0_f64 / t2371;
    let t2373 = t216 * t2372;
    let t2382 = 1.0_f64 / t217 / t214;
    let t2386 = 4.0_f64 / 9.0_f64 * t2257;
    let t2394 = 0.39862222222222222223e0_f64 * t2257;
    let t2399 = 1.0_f64/f64::sqrt(t214);
    let t2404 = 0.13692777777777777778e0_f64 * t2280;
    let t2414 = t777 * t777;
    (t2371, t2372, t2373, t2382, t2386, t2394, t2399, t2404, t2414)
}
