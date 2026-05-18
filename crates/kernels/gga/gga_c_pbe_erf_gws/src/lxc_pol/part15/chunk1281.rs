//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1281/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1281<F: Float>(t3958: F, t6148: F, t352: F, t830: F, t9292: F, t3965: F, t8649: F, t14136: F, t8700: F, t4002: F, t8746: F, t14696: F, t29287: F, t3972: F, t3975: F) -> (F, F, F, F, F, F) {
    let t53840 = t3958 * t6148;
    let t53841 = t830 * t352;
    let t53843 = t53840 * t53841 * t9292;
    let t53846 = t3965 * t8649;
    let t53848 = t14136 * t8700;
    let t53852 = t8746 * t4002;
    let t53856 = t3972 * t3975 * t29287 * t14696;
    (t53841, t53843, t53846, t53848, t53852, t53856)
}
