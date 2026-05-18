//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1272/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1272<F: Float>(t3892: F, t859: F, t13792: F, t1114: F, t332: F, t3747: F, t13793: F, t14617: F, t53229: F, t53571: F, t3912: F, t51580: F) -> (F, F, F, F, F, F) {
    let t56100 = t859 * t3892;
    let t56101 = t13792 * t56100;
    let t56104 = t1114 * t3747 * t332;
    let t56105 = t56104 * t13793;
    let t56107 = t53229 * t14617;
    let t56110 = t53571 * t14617;
    let t56112 = t3912 * t51580;
    (t56101, t56104, t56105, t56107, t56110, t56112)
}
