//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 991/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk991<F: Float>(t10400: F, t10405: F, t10410: F, t10413: F, t10417: F, t10421: F, t10423: F, t10428: F, t10432: F, t10436: F, t10441: F, t10446: F, t5919: F, t5922: F, t7190: F, t7193: F, t8425: F) -> F {
    let t11202 = -t10400 + t10405 + t10410 - t10413 + t10417 + t10421 + t10423 + F::new(0.22363485482220676312e-1) * t8425 - t5919 + t5922 - t7190 + t7193 + t10428 + t10432 - t10436 - t10441 + t10446;
    t11202
}
