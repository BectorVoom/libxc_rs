//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1130/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1130<F: Float>(t3083: F, t4002: F, t1105: F, t1193: F, t353: F, t4386: F) -> (F, F, F, F) {
    let t14400 = t3083 * t4002;
    let t14402 = t1193 * t1105;
    let t14403 = t353 * t14402;
    let t14404 = t4386 * t14403;
    (t14400, t14402, t14403, t14404)
}
