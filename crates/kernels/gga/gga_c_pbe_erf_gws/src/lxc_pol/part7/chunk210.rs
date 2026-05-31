//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 210/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk210<F: Float>(t145: F, t153: F, t395: F, t401: F, t542: F) -> F {
    let t545 = -F::cast_from(0.1725e-2_f64) * t395 - F::cast_from(0.13655e-1_f64) * t401 + F::cast_from(0.30486129349252551566e-2_f64) * t145 - F::cast_from(0.46475e-3_f64) * t153 * t542;
    t545
}
