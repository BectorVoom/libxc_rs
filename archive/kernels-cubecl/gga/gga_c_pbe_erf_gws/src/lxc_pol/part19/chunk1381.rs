//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1381/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1381<F: Float>(t54126: F, t56954: F, t56956: F, t56958: F, t56960: F, t56962: F, t56964: F, t56966: F, t56968: F, t56970: F, t56972: F, t56975: F) -> F {
    let t58645 = t56954 / F::cast_from(12.0_f64) - t56956 / F::cast_from(24.0_f64) + t56958 / F::cast_from(64.0_f64) - t56960 / F::cast_from(24.0_f64) + t56962 / F::cast_from(48.0_f64) - t56964 / F::cast_from(192.0_f64) + F::cast_from(5.0_f64) / F::cast_from(48.0_f64) * t56966 + t56968 / F::cast_from(12.0_f64) + t56970 / F::cast_from(192.0_f64) - t56972 / F::cast_from(192.0_f64) + t56975 / F::cast_from(48.0_f64) + F::cast_from(119.0_f64) / F::cast_from(864.0_f64) * t54126;
    t58645
}
