//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1154/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1154(t2389: f64, t6700: f64, t6696: f64, t1441: f64, t9264: f64, t1429: f64, t2365: f64, t2366: f64, t6393: f64, t21074: f64, t901: f64, t20675: f64, t9538: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31213 = 0.11916829983950142223e0_f64 * t6700 * t2389;
    let t31215 = 0.11916829983950142223e0_f64 * t6696 * t2389;
    let t31216 = t1441 * t9264;
    let t31291 = 0.29792074959875355558e-1_f64 * t1429 * t2365 * t2366 * t6393;
    let t31299 = 0.29792074959875355558e-1_f64 * t21074 * t901;
    let t31346 = t20675 * t9538;
    (t31213, t31215, t31216, t31291, t31299, t31346)
}
