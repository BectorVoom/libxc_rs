//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 725/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk725<F: Float>(t5408: F, t5410: F, t5412: F, t5415: F, t5458: F, t5460: F, t5462: F, t5466: F, t5469: F, t5472: F, t5474: F, t5476: F, t5479: F, t5483: F, t5487: F, t5492: F, t5496: F) -> F {
    let t5958 = t5408 + t5410 + t5412 + t5415 + t5458 + t5460 - t5462 - t5466 + t5469 + t5472 + t5474 + t5476 + t5479 + t5483 + t5487 + t5492 - t5496;
    t5958
}
