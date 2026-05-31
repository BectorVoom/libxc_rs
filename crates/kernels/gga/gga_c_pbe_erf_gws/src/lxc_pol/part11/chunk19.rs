//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 19/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk19<F: Float>(t11: F, t14: F, t17: F, t25: F) -> (F, F, F) {
    let t27 = F::cast_from(0.379785e1_f64) * t14 + F::cast_from(0.8969e0_f64) * t11 + F::cast_from(0.204775e0_f64) * t17 + F::cast_from(0.123235e0_f64) * t25;
    let t30 = F::cast_from(1.0_f64) + F::cast_from(0.16081824322151104822e2_f64) / t27;
    let t31 = F::ln(t30);
    (t27, t30, t31)
}
