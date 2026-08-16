//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1112/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1112(t12213: f64, t2530: f64, t1457: f64, t2103: f64, t13861: f64, t4614: f64, t833: f64, t2660: f64, t39146: f64, t43465: f64, t43468: f64, t43471: f64, t43477: f64, t43479: f64, t43481: f64, t43484: f64, t43489: f64, t43492: f64) -> (f64, f64) {
    let t47225 = t12213 * t2530;
    let t47227 = t2103 * t1457 * t47225;
    let t47230 = t833 * t4614 * t13861;
    let t47234 = t39146 * t2660;
    let t47237 = 0.71500979903700853338e0_f64 * t47227 + 0.15337170381568299871e2_f64 * t47230 + t43465 + t43468 + t43471 - t43477 - t43479 - 0.10725146985555128001e1_f64 * t43481 - 0.10725146985555128001e1_f64 * t43484 - t43489 + 0.10725146985555128001e1_f64 * t47234 - 0.92023022289409799224e1_f64 * t43492;
    (t47225, t47237)
}
