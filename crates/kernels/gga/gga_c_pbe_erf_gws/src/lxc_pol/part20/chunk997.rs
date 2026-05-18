//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 997/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk997<F: Float>(t10738: F, t10739: F, t10741: F, t10745: F, t10749: F, t10751: F, t10837: F, t10838: F, t10840: F, t5359: F, t7578: F, t7617: F, t7619: F, t7623: F, t7665: F, t7668: F) -> F {
    let t11216 = t10738 + t7578 - t10739 - t10741 - t10745 + t10749 + t10751 - t10837 + t7617 + t7619 + t7623 - t10838 + t5359 - t10840 - t7665 - t7668;
    t11216
}
