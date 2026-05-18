//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1234/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1234<F: Float>(t1112: F, t361: F, t51543: F, t13925: F, t22493: F, t1178: F, t2079: F, t898: F, t14664: F, t9270: F, t14705: F, t51666: F) -> (F, F, F, F, F, F) {
    let t53138 = t361 * t51543 * t1112;
    let t53155 = F::new(7.0) / F::new(144.0) * t22493 * t13925;
    let t53156 = t1178 * t51543;
    let t53161 = t1178 * t898 * t2079;
    let t53177 = F::new(7.0) / F::new(72.0) * t9270 * t14664;
    let t53178 = t51666 * t14705;
    (t53138, t53155, t53156, t53161, t53177, t53178)
}
