//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1070/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1070<F: Float>(t13832: F, t51963: F, t51649: F, t867: F, t3966: F, t326: F, t378: F, t6594: F, t13968: F, t14001: F, t745: F, t837: F, t833: F, t850: F, t851: F, t4002: F, t4424: F) -> (F, F, F, F, F, F, F, F) {
    let t51964 = t51963 * t13832;
    let t51966 = t51649 * t867;
    let t51967 = t51966 * t3966;
    let t51977 = t326 * t6594 * t378;
    let t51978 = 455.0 / 1296.0 * t51977;
    let t51979 = t14001 * t13968;
    let t51989 = t745 * t837;
    let t51992 = t850 * t851 * t51989 * t833;
    let t52020 = t4424 * t4002;
    (t51964, t51966, t51967, t51978, t51979, t51989, t51992, t52020)
}
