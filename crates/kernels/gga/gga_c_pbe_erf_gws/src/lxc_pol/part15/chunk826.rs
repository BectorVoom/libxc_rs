//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 826/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk826<F: Float>(t1403: F, t7899: F, t2559: F, t1820: F, t2756: F, t579: F, t532: F, t4358: F, t4561: F) -> (F, F, F) {
    let t7900 = t7899 * t1403;
    let t7901 = t2559 * t7900;
    let t7903 = 8.0 / 27.0 * t1820 * t7901;
    let t7905 = 8.0 / 45.0 * t579 * t2756;
    let t7906 = 4.0 * t532;
    let t7907 = 12.0 * t4358;
    let t7908 = -t7906 - t7907 + t4561;
    (t7903, t7905, t7908)
}
