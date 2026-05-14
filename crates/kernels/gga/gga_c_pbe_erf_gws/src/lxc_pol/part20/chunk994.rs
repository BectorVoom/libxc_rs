//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 994/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk994<F: Float>(t13953: F, t3966: F, t2367: F, t4002: F, t3979: F, t3997: F, t1176: F, t903: F, t923: F) -> (F, F, F, F) {
    let t13954 = t13953 * t3966;
    let t13962 = t2367 * t4002;
    let t13964 = t3979 * t3997;
    let t13972 = t1176 * t923 * t903;
    (t13954, t13962, t13964, t13972)
}
