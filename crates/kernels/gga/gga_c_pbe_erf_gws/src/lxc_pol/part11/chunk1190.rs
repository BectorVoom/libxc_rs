//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1190/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1190<F: Float>(t225: F, t231: F, t48321: F, t48369: F, t48373: F, t48377: F, t48380: F, t48381: F, t48382: F, t48387: F, t48392: F, t48393: F, t48394: F) -> F {
    let t48694 = t48369 + t48373 + t48377 + t48380 - t48381 + t48382 + t48387 - t48392 + t48393 + F::new(4.0) / F::new(3.0) * t48321 * t225 * t231 - t48394;
    t48694
}
