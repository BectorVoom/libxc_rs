//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1201/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1201<F: Float>(t13778: F, t587: F, t589: F, t1457: F, t1572: F, t46945: F, t13728: F, t4673: F, t42279: F, t42282: F, t42284: F, t42288: F, t42292: F, t42298: F, t42305: F, t42309: F, t42312: F) -> F {
    let t48121 = t587 * t589 * t13778;
    let t48124 = t1572 * t1457 * t46945;
    let t48127 = t1572 * t4673 * t13728;
    let t48129 = F::cast_from(0.71500979903700853338e0_f64) * t42279 + F::cast_from(0.25561950635947166451e0_f64) * t48121 + F::cast_from(0.71500979903700853338e0_f64) * t48124 + t42282 - t42284 - t42288 - t42292 - t42298 + F::cast_from(0.47667319935800568892e0_f64) * t48127 + t42305 - t42309 - t42312;
    t48129
}
