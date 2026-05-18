//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 699/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk699<F: Float>(t2409: F, t3067: F, t4016: F, t331: F, t345: F, t56: F, t859: F) -> (F, F, F, F) {
    let t4018 = t2409 * t3067 * t4016;
    let t4021 = t345 * t331;
    let t4022 = t4021 * t56;
    let t4023 = t4022 * t859;
    (t4018, t4021, t4022, t4023)
}
