//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1028/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1028<F: Float>(t42341: F, t42350: F, t42354: F, t42356: F, t42359: F, t42367: F, t42370: F, t42373: F, t42376: F, t42379: F, t48142: F, t48143: F, t48144: F, t48149: F, t48154: F, t48157: F, t48160: F, t48167: F, t48172: F, t48175: F) -> F {
    let t50911 = t42341 - t48142 + t48143 - t42350 + F::cast_from(0.14300195980740170668e1_f64) * t48144 + t42354 - F::cast_from(0.13803453343411469884e2_f64) * t48149 + t42356 - t42359 + F::cast_from(0.85206502119823888169e-1_f64) * t48154 - F::cast_from(0.89376224879626066674e-1_f64) * t48157 + F::cast_from(0.59584149919750711116e-1_f64) * t48160 + t42367 + t42370 + t42373 - t42376 + t42379 - F::cast_from(0.71500979903700853338e0_f64) * t48167 + F::cast_from(0.85801175884441024008e1_f64) * t48172 - F::cast_from(0.23005755572352449806e2_f64) * t48175;
    t50911
}
