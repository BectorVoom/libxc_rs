//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 690/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk690<F: Float>(t5498: F, t5504: F, t5508: F, t5512: F, t5514: F, t5518: F, t5526: F, t5528: F, t5532: F, t5535: F, t5538: F, t5542: F, t5547: F, t5553: F, t5555: F, t5558: F, t5565: F) -> (F,) {
    let t5959 = -t5498 - t5504 - t5508 + t5512 - t5514 + t5518 - t5526 + t5528 - t5532 - t5535 + t5538 + t5542 - t5547 + t5553 - t5555 - t5558 + t5565;
    (t5959,)
}
