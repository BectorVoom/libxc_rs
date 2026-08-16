//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 902/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk902<F: Float>(t10063: F, t128: F, t10: F, t156: F, t3656: F, t496: F, t3660: F, t3665: F, t501: F, t395: F, t3668: F, t481: F) -> (F, F, F, F, F, F, F, F) {
    let t10064 = t128 * t10063;
    let t10065 = t10 * t10064;
    let t10068 = t156 * t3656;
    let t10069 = t496 * t10068;
    let t10071 = t156 * t3660;
    let t10072 = t496 * t10071;
    let t10074 = t501 * t3665;
    let t10075 = t10074 * t395;
    let t10077 = t501 * t3668;
    let t10078 = t10077 * t395;
    let t10081 = t3665 * t481;
    (t10065, t10068, t10069, t10071, t10072, t10075, t10078, t10081)
}
