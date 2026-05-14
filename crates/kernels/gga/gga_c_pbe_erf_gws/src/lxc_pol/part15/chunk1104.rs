//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1104/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1104<F: Float>(t14121: F, t9215: F, t13917: F, t14423: F, t3223: F, t361: F, t874: F, t14724: F, t343: F, t13783: F, t14809: F, t4414: F, t14469: F, t50943: F, t50936: F, t3972: F, t3975: F, t9410: F) -> (F, F, F, F, F, F, F) {
    let t53487 = t14121 * t9215;
    let t53493 = t13917 * t361 * t14423 * t874 * t3223;
    let t53496 = t361 * t14724 * t343;
    let t53498 = t13917 * t53496 * t13783;
    let t53503 = 7.0 / 72.0 * t4414 * t14809;
    let t53508 = t50943 * t14469;
    let t53509 = 7.0 / 72.0 * t53508;
    let t53510 = t50936 * t14469;
    let t53513 = t3972 * t3975 * t9410;
    (t53487, t53493, t53498, t53503, t53509, t53510, t53513)
}
