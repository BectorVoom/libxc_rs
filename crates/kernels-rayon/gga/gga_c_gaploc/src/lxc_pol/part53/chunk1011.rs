//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1011/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1011(t2392: f64, t48171: f64, t47953: f64, t6710: f64, t6711: f64, t12092: f64, t2478: f64, t6583: f64, t1457: f64, t46915: f64, t557: f64, t1572: f64, t46920: f64) -> (f64, f64, f64, f64, f64) {
    let t48172 = t48171 * t2392;
    let t48175 = t6710 * t6711 * t47953;
    let t48178 = t6583 * t12092 * t2478;
    let t48182 = 0.10725146985555128001e1_f64 * t557 * t1457 * t46915;
    let t48185 = 0.71500979903700853338e0_f64 * t1572 * t1457 * t46920;
    (t48172, t48175, t48178, t48182, t48185)
}
