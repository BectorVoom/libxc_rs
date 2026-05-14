//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 960/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk960<F: Float>(t11990: F, t4386: F, t2168: F, t2127: F, t3781: F, t850: F, t860: F, t2142: F, t3788: F, t326: F, t9385: F, t6252: F, t3037: F, t5: F, t337: F, t2121: F) -> (F, F, F, F, F, F, F, F) {
    let t12032 = t4386 * t11990;
    let t12034 = t2168 * t12032 / 24.0;
    let t12036 = t850 * t3781 * t2127;
    let t12038 = t12036 * t860 / 96.0;
    let t12039 = t3788 * t2142;
    let t12040 = 7.0 / 288.0 * t12039;
    let t12041 = t326 * t9385;
    let t12042 = t12041 * t6252;
    let t12043 = t5 * t3037;
    let t12044 = t337 * t12043;
    let t12045 = t2121 * t12044;
    (t12034, t12036, t12038, t12040, t12041, t12042, t12044, t12045)
}
