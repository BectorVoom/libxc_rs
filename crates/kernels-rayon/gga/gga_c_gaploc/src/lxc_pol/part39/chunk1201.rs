//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1201/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1201(t13778: f64, t587: f64, t589: f64, t1457: f64, t1572: f64, t46945: f64, t13728: f64, t4673: f64, t42279: f64, t42282: f64, t42284: f64, t42288: f64, t42292: f64, t42298: f64, t42305: f64, t42309: f64, t42312: f64) -> f64 {
    let t48121 = t587 * t589 * t13778;
    let t48124 = t1572 * t1457 * t46945;
    let t48127 = t1572 * t4673 * t13728;
    let t48129 = 0.71500979903700853338e0_f64 * t42279 + 0.25561950635947166451e0_f64 * t48121 + 0.71500979903700853338e0_f64 * t48124 + t42282 - t42284 - t42288 - t42292 - t42298 + 0.47667319935800568892e0_f64 * t48127 + t42305 - t42309 - t42312;
    t48129
}
