//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 793/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk793<F: Float>(t10894: F, t4947: F, t2640: F, t241: F, t4780: F, t2586: F, t4975: F, t893: F, t4979: F, t4971: F, t1382: F, t2595: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14252 = t10894 * t4947;
    let t14253 = t2640 * t14252;
    let t14267 = t241 * t4780;
    let t14279 = t2586 * t4975;
    let t14280 = t893 * t14279;
    let t14284 = t2586 * t4979;
    let t14285 = t893 * t14284;
    let t14289 = t2586 * t4971;
    let t14290 = t893 * t14289;
    let t14292 = t2595 * t1382;
    (t14253, t14267, t14279, t14280, t14284, t14285, t14289, t14290, t14292)
}
