//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 799/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk799<F: Float>(t12917: F, t12919: F, t12921: F, t12923: F, t12925: F, t12927: F) -> F {
    let t12929 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t12917 - t12919 / F::cast_from(3.0_f64) + t12921 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t12923 - t12925 / F::cast_from(3.0_f64) + t12927 / F::cast_from(3.0_f64);
    t12929
}
