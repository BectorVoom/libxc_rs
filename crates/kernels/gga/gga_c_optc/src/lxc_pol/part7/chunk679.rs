//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 679/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk679<F: Float>(t517: F, t6369: F, t11: F, t2: F, t39: F, t1776: F, t6366: F, t525: F, t3649: F, t3696: F, t6364: F, t6367: F) -> (F, F, F, F, F, F) {
    let t6370 = t517 * t6369;
    let t6373 = F::new(1.0)/pow_3_2::<F>(t11);
    let t6374 = t6373 * t2;
    let t6375 = t6374 * t39;
    let t6377 = t1776 * t6366;
    let t6379 = t525 * t6369;
    let t6382 = -F::new(0.47063e1) * t6364 + F::cast_from(0.31375333333333333334e1_f64) * t6367 - F::cast_from(0.36604555555555555556e1_f64) * t6370 - F::cast_from(0.16068111111111111111e1_f64) * t3649 + F::cast_from(0.28051666666666666666e0_f64) * t6375 - F::cast_from(0.56103333333333333332e0_f64) * t6377 - F::cast_from(0.6545388888888888889e0_f64) * t6379 - F::cast_from(0.46308888888888888888e0_f64) * t3696;
    (t6370, t6374, t6375, t6377, t6379, t6382)
}
