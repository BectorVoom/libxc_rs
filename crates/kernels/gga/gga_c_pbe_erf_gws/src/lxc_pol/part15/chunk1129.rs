//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1129/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1129<F: Float>(t3965: F, t8736: F, t14784: F, t50994: F, t20091: F, t4157: F, t14637: F, t3974: F, t3990: F, t8804: F, t2409: F, t26647: F, t3959: F, t8723: F, t3202: F, t3955: F) -> (F, F, F, F, F, F, F) {
    let t53950 = t3965 * t8736;
    let t53952 = t50994 * t14784;
    let t53953 = 7.0 / 288.0 * t53952;
    let t53959 = t20091 * t4157;
    let t53963 = t14637 * t3990 * t3974 * t8804;
    let t53966 = t3959 * t2409 * t26647;
    let t53968 = t3959 * t8723;
    let t53970 = t3955 * t3202;
    (t53950, t53953, t53959, t53963, t53966, t53968, t53970)
}
