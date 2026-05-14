//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1105/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1105<F: Float>(t3965: F, t8736: F, t14784: F, t50994: F, t14637: F, t3974: F, t3990: F, t8804: F, t2409: F, t26647: F, t3959: F, t8723: F, t3202: F, t3955: F, t14121: F, t26768: F) -> (F, F, F, F, F, F, F) {
    let t53950 = t3965 * t8736;
    let t53952 = t50994 * t14784;
    let t53963 = t14637 * t3990 * t3974 * t8804;
    let t53966 = t3959 * t2409 * t26647;
    let t53968 = t3959 * t8723;
    let t53970 = t3955 * t3202;
    let t53973 = t14121 * t2409 * t26768;
    (t53950, t53952, t53963, t53966, t53968, t53970, t53973)
}
