//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 330/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk330<F: Float>(t127: F, t488: F, t495: F, t496: F, t504: F, t973: F, t975: F, t978: F, t133: F, t517: F) -> (F, F) {
    let t981 = -t488 - t973 - t495 - t496 * t975 / F::new(2.0) - t504 - F::new(0.146904e1) * t127 * t978;
    let t985 = -t488 - t973 - t517 - F::new(0.1724255e1) * t133 * t975;
    (t981, t985)
}
