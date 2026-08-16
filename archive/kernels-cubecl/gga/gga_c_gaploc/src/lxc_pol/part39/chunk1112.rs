//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1112/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1112<F: Float>(t12213: F, t2530: F, t1457: F, t2103: F, t13861: F, t4614: F, t833: F, t2660: F, t39146: F, t43465: F, t43468: F, t43471: F, t43477: F, t43479: F, t43481: F, t43484: F, t43489: F, t43492: F) -> (F, F) {
    let t47225 = t12213 * t2530;
    let t47227 = t2103 * t1457 * t47225;
    let t47230 = t833 * t4614 * t13861;
    let t47234 = t39146 * t2660;
    let t47237 = F::cast_from(0.71500979903700853338e0_f64) * t47227 + F::cast_from(0.15337170381568299871e2_f64) * t47230 + t43465 + t43468 + t43471 - t43477 - t43479 - F::cast_from(0.10725146985555128001e1_f64) * t43481 - F::cast_from(0.10725146985555128001e1_f64) * t43484 - t43489 + F::cast_from(0.10725146985555128001e1_f64) * t47234 - F::cast_from(0.92023022289409799224e1_f64) * t43492;
    (t47225, t47237)
}
