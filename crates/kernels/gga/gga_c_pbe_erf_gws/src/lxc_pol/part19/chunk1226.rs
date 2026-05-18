//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1226/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1226<F: Float>(t1112: F, t361: F, t51543: F, t1178: F, t2079: F, t898: F, t14705: F, t51666: F, t14633: F, t1114: F, t50942: F, t13984: F) -> (F, F, F, F, F, F, F) {
    let t53138 = t361 * t51543 * t1112;
    let t53156 = t1178 * t51543;
    let t53161 = t1178 * t898 * t2079;
    let t53178 = t51666 * t14705;
    let t53198 = t51666 * t14633;
    let t53229 = t1114 * t50942;
    let t53230 = t53229 * t13984;
    (t53138, t53156, t53161, t53178, t53198, t53229, t53230)
}
