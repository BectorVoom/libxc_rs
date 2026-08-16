//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1205/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1205<F: Float>(t42356: F, t42359: F, t42363: F, t42367: F, t42370: F, t42373: F, t42376: F, t42379: F, t48149: F, t48154: F, t48157: F, t48160: F) -> F {
    let t48162 = -F::cast_from(0.69017266717057349418e1_f64) * t48149 + t42356 - t42359 + F::cast_from(0.43710935587469654631e2_f64) * t42363 + F::cast_from(0.42603251059911944084e-1_f64) * t48154 - F::cast_from(0.44688112439813033337e-1_f64) * t48157 + F::cast_from(0.29792074959875355558e-1_f64) * t48160 + t42367 + t42370 + t42373 - t42376 + t42379;
    t48162
}
