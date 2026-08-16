//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 510/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk510(t7030: f64, t9305: f64, t1424: f64, t4386: f64, t4391: f64, t9266: f64, t9270: f64, t9276: f64, t9281: f64, t9282: f64, t9289: f64, t9291: f64, t9296: f64, t9298: f64, t9302: f64) -> (f64, f64) {
    let t9307 = 0.29792074959875355558e-1_f64 * t9305 * t7030;
    let t9308 = -t9266 + t9270 - t9276 - t9281 - 0.79445533226334281487e-1_f64 * t4391 * t9282 + t9289 + 0.11916829983950142223e0_f64 * t9291 * t4386 + t9296 - 0.39722766613167140743e-1_f64 * t9298 * t1424 - 0.39722766613167140743e-1_f64 * t9302 * t1424 - t9307;
    (t9307, t9308)
}
