//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1354/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1354<F: Float>(t54886: F, t56061: F, t56067: F, t56070: F, t56074: F, t56077: F, t56080: F, t56093: F, t56098: F, t56101: F, t56105: F, t56107: F, t56110: F, t57958: F, t57972: F, t827: F) -> F {
    let t57974 = t56061 / F::cast_from(24.0_f64) - t827 * t57958 / F::cast_from(96.0_f64) + t56067 / F::cast_from(192.0_f64) + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t56070 - t56074 / F::cast_from(768.0_f64) - t56077 / F::cast_from(96.0_f64) - t56080 / F::cast_from(96.0_f64) - t54886 - t56093 / F::cast_from(48.0_f64) - t56098 / F::cast_from(192.0_f64) - t56101 / F::cast_from(24.0_f64) - t56105 / F::cast_from(24.0_f64) + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t56107 - t56110 / F::cast_from(24.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t57972;
    t57974
}
