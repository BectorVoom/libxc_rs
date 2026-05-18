//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1235/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1235<F: Float>(t3952: F, t8751: F, t14423: F, t14682: F, t2158: F, t3989: F, t14617: F, t50943: F, t3990: F, t3991: F, t9080: F, t345: F, t6126: F) -> (F, F, F, F, F) {
    let t53266 = t3952 * t8751;
    let t53270 = t3989 * t14682 * t14423 * t2158;
    let t53272 = t50943 * t14617;
    let t53276 = t3989 * t3990 * t3991 * t9080;
    let t53283 = t345 * t6126;
    (t53266, t53270, t53272, t53276, t53283)
}
