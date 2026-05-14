//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 747/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk747<F: Float>(t2209: F, t337: F, t2118: F, t2365: F, t274: F, t4394: F, t828: F, t2137: F, t2132: F, t2271: F, t814: F, t816: F, t362: F, t922: F, t2276: F, t932: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6148 = t2209 * t337;
    let t6154 = t2118 * t2365;
    let t6158 = t4394 * t274;
    let t6183 = t2365 * t828;
    let t6184 = t6183 * t2137;
    let t6187 = t2271 * t2132;
    let t6196 = t816 * t814;
    let t6201 = t362 * t922;
    let t6203 = t2276 * t6201 * t932;
    (t6148, t6154, t6158, t6183, t6184, t6187, t6196, t6201, t6203)
}
