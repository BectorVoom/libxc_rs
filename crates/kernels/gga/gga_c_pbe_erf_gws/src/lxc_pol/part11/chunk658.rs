//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 658/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk658<F: Float>(t2053: F, t3928: F, t3342: F, t4351: F, t3351: F, t4366: F, t2358: F, t3916: F, t3703: F, t831: F, t6148: F, t830: F) -> (F, F, F, F, F) {
    let t9772 = t3928 * t2053;
    let t9778 = t4351 * t3342;
    let t9793 = t4366 * t3351;
    let t9815 = t3916 * t2358;
    let t9818 = t831 * t3703;
    let t9820 = t6148 * t830 * t9818;
    (t9772, t9778, t9793, t9815, t9820)
}
