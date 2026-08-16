//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 591/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk591(t492: f64, t8259: f64, t500: f64, t2275: f64, t6382: f64, t2271: f64, t2279: f64, t499: f64, t8072: f64, t498: f64, t4235: f64, t4231: f64, t8077: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8260 = t8259 * t492;
    let t8261 = t8260 * t500;
    let t8263 = t6382 * t2275;
    let t8265 = t2271 * t2279;
    let t8267 = t499 * t8072;
    let t8268 = t498 * t8267;
    let t8269 = t4235 * t8268;
    let t8271 = t4231 * t8077;
    (t8260, t8261, t8263, t8265, t8268, t8269, t8271)
}
