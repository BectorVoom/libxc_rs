//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1246/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1246<F: Float>(t14657: F, t50891: F, t1114: F, t51916: F, t51919: F, t50935: F, t13793: F, t1112: F, t2306: F, t3074: F, t833: F, t837: F) -> (F, F, F, F, F) {
    let t53564 = t14657 * t50891;
    let t53566 = t1114 * t51916;
    let t53567 = t53566 * t51919;
    let t53571 = t1114 * t50935;
    let t53572 = t53571 * t13793;
    let t53577 = t3074 * t2306 * t1112 * t837 * t833;
    (t53564, t53567, t53571, t53572, t53577)
}
