//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1381/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1381<F: Float>(t54126: F, t56954: F, t56956: F, t56958: F, t56960: F, t56962: F, t56964: F, t56966: F, t56968: F, t56970: F, t56972: F, t56975: F) -> F {
    let t58645 = t56954 / F::new(12.0) - t56956 / F::new(24.0) + t56958 / F::new(64.0) - t56960 / F::new(24.0) + t56962 / F::new(48.0) - t56964 / F::new(192.0) + F::new(5.0) / F::new(48.0) * t56966 + t56968 / F::new(12.0) + t56970 / F::new(192.0) - t56972 / F::new(192.0) + t56975 / F::new(48.0) + F::new(119.0) / F::new(864.0) * t54126;
    t58645
}
