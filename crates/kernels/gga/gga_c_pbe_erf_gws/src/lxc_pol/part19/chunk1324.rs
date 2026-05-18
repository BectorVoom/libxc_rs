//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1324/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1324<F: Float>(t3837: F, t51301: F, t11585: F, t4028: F, t11693: F, t51274: F, t14058: F, t3875: F, t36666: F, t850: F, t14093: F, t11849: F, t14031: F) -> (F, F, F, F, F, F) {
    let t57182 = t51301 * t3837;
    let t57184 = t4028 * t11585;
    let t57186 = t51274 * t11693;
    let t57188 = t14058 * t3875;
    let t57190 = t850 * t36666;
    let t57191 = t57190 * t14093;
    let t57195 = t14031 * t11849;
    (t57182, t57184, t57186, t57188, t57191, t57195)
}
