//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1228/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1228<F: Float>(t1161: F, t13781: F, t2079: F, t3972: F, t9370: F, t1123: F, t2313: F, t50998: F, t51021: F, t938: F, t14767: F, t2373: F) -> (F, F, F) {
    let t53065 = t3972 * t13781 * t1161 * t2079 * t9370;
    let t53072 = t50998 * t51021 * t1123 * t2313 * t938;
    let t53126 = t14767 * t2373;
    (t53065, t53072, t53126)
}
