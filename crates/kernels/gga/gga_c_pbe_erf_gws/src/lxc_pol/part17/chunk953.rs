//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 953/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk953<F: Float>(t9351: F, t9362: F, t9409: F, t9430: F, t9456: F, t9475: F, t9503: F, t9530: F, t9536: F, t9559: F, t9567: F, t9584: F, t9599: F, t9623: F, t9664: F, t9684: F) -> (F,) {
    let t9688 = t9351 + t9362 + t9409 + t9430 + t9456 + t9475 + t9503 + t9530 + t9536 + t9559 + t9567 + t9584 + t9599 + t9623 + t9664 + t9684;
    (t9688,)
}
