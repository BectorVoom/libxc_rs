//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 969/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk969<F: Float>(t43476: F, t45356: F, t45357: F, t45358: F, t45359: F, t45366: F, t45367: F, t45372: F, t45375: F, t45377: F, t45379: F, t45381: F, t45383: F, t45385: F, t45387: F, t45390: F, t45392: F, t45394: F, t45397: F, t45408: F) -> F {
    let t50136 = -t45356 + t45357 + t45358 + t45359 - F::cast_from(0.12780975317973583225e0_f64) * t43476 + t45366 + t45367 - t45372 - t45375 - t45377 + t45379 + t45381 + t45383 - t45385 + t45387 - t45390 + t45392 + t45394 + t45397 + t45408;
    t50136
}
