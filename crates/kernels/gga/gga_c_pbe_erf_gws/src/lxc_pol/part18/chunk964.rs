//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 964/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk964<F: Float>(t1076: F, t1112: F, t2118: F, t3074: F, t1185: F, t346: F, t825: F, t11478: F, t3139: F, t3140: F, t3138: F, t875: F, t2168: F, t11994: F, t2255: F, t2279: F) -> (F, F, F, F, F, F, F, F) {
    let t12072 = t1112 * t1076;
    let t12073 = t2118 * t12072;
    let t12074 = t3074 * t12073;
    let t12076 = t346 * t825 * t1185;
    let t12078 = t12074 * t12076 / 96.0;
    let t12080 = t3139 * t11478 * t3140;
    let t12082 = t3138 * t12080 / 16.0;
    let t12084 = t3139 * t11478 * t875;
    let t12086 = t2168 * t12084 / 96.0;
    let t12088 = t2255 * t11994 * t2279;
    (t12072, t12074, t12078, t12080, t12082, t12084, t12086, t12088)
}
