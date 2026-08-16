//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 866/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk866(t4044: f64, t7178: f64, t3608: f64, t2274: f64, t2436: f64, t2435: f64, t2432: f64, t2554: f64, t2352: f64, t984: f64, t92: f64, t93: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8272 = t4044 * t7178;
    let t8273 = t3608 * t8272;
    let t8276 = t2436 * t2274;
    let t8277 = t2435 * t8276;
    let t8280 = t2554 * t2432;
    let t8283 = t984 * t2352;
    let t8285 = t92 * t92;
    let t8287 = 1.0_f64 / t8285 * t93;
    (t8272, t8273, t8276, t8277, t8280, t8283, t8285, t8287)
}
