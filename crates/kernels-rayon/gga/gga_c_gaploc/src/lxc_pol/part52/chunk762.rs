//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 762/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk762(t11576: f64, t795: f64, t313: f64, t1: f64, t36610: f64, t2021: f64, t2089: f64, t106: f64, t316: f64, t11286: f64, t501: f64, t11401: f64, t540: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37060 = t795 * t11576;
    let t37061 = t313 * t37060;
    let t37179 = t36610 * t1;
    let t37180 = t2021 * t37179;
    let t37200 = t2089 * t11576;
    let t37218 = t11576 * t1 * t106 * t316;
    let t37275 = t11286 * t501;
    let t37326 = t11401 * t540;
    (t37060, t37061, t37180, t37200, t37218, t37275, t37326)
}
