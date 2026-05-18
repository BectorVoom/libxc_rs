//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1203/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1203<F: Float>(t42315: F, t42316: F, t42350: F, t42354: F, t48131: F, t48134: F, t48137: F, t48140: F, t48141: F, t48142: F, t48143: F, t48144: F) -> F {
    let t48146 = F::new(0.11502877786176224903e2) * t48131 + F::new(0.11502877786176224903e2) * t48134 + F::new(0.11502877786176224903e2) * t48137 - t42315 - F::new(0.14896037479937677779e-1) * t42316 + t48140 + t48141 - t48142 + t48143 - t42350 + F::new(0.71500979903700853338e0) * t48144 + t42354;
    t48146
}
