//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 990/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk990<F: Float>(t11964: F, t3137: F, t311: F, t9741: F, t11417: F, t277: F, t128: F, t2546: F, t2761: F, t1026: F, t761: F, t1093: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11965 = t11964 * t3137;
    let t11966 = t311 * t11965;
    let t11967 = t11966 * t9741;
    let t11969 = t277 * t11417;
    let t11970 = t2546 * t128;
    let t11971 = t2761 * t11970;
    let t11972 = t11969 * t11971;
    let t11974 = t761 * t1026;
    let t11975 = t11974 * t1093;
    (t11965, t11966, t11967, t11969, t11970, t11971, t11972, t11974, t11975)
}
