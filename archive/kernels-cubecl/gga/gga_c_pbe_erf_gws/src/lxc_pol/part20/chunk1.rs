//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1<F: Float>() -> F {
    let pi = F::cast_from(M_PI);
    let t2 = F::cast_from(1.0_f64) / pi;
    t2
}
