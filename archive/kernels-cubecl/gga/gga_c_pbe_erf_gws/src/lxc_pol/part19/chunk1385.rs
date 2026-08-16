//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1385/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1385<F: Float>(t55547: F, t55548: F, t57060: F, t57062: F, t57064: F, t57066: F, t57068: F, t57070: F, t57073: F, t57075: F, t57077: F, t57079: F) -> F {
    let t58697 = -t57060 / F::cast_from(12.0_f64) - t57062 / F::cast_from(96.0_f64) - t57064 / F::cast_from(24.0_f64) + t57066 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t57068 + t57070 / F::cast_from(96.0_f64) + t55547 - t57073 / F::cast_from(48.0_f64) - t57075 / F::cast_from(96.0_f64) - t57077 / F::cast_from(12.0_f64) - t55548 - t57079 / F::cast_from(48.0_f64);
    t58697
}
