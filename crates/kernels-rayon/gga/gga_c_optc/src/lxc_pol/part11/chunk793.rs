//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 793/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk793(t10894: f64, t4947: f64, t2640: f64, t241: f64, t4780: f64, t2586: f64, t4975: f64, t893: f64, t4979: f64, t4971: f64, t1382: f64, t2595: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
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
