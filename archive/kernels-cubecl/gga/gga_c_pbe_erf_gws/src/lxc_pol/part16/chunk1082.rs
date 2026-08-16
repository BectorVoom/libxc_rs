//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1082/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1082<F: Float>(t1177: F, t13899: F, t1178: F, t2418: F, t371: F, t2338: F, t3975: F, t3972: F, t915: F, t3970: F) -> (F, F, F, F, F, F, F) {
    let t13900 = t1177 * t13899;
    let t13903 = t371 * t1178 * t2418;
    let t13904 = t1177 * t13903;
    let t13906 = t3975 * t2338;
    let t13907 = t3972 * t13906;
    let t13916 = t915 * t371;
    let t13917 = t3970 * t13916;
    (t13900, t13903, t13904, t13906, t13907, t13916, t13917)
}
