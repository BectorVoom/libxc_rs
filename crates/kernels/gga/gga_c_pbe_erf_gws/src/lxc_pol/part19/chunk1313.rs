//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1313/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1313<F: Float>(t11758: F, t4039: F, t11531: F, t14015: F, t8991: F, t9035: F, t11754: F, t2080: F, t3107: F, t12044: F, t14092: F, t38537: F) -> (F, F, F, F, F, F) {
    let t57017 = t4039 * t11758;
    let t57019 = t14015 * t11531;
    let t57021 = t9035 * t8991;
    let t57023 = t4039 * t11754;
    let t57026 = t2080 * t3107;
    let t57028 = t57026 * t14092 * t12044;
    let t57030 = t2080 * t38537;
    (t57017, t57019, t57021, t57023, t57028, t57030)
}
