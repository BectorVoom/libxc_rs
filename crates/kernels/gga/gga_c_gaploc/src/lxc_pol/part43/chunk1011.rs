//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1011/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1011<F: Float>(t2392: F, t48171: F, t47953: F, t6710: F, t6711: F, t12092: F, t2478: F, t6583: F, t1457: F, t46915: F, t557: F, t1572: F, t46920: F) -> (F, F, F, F, F) {
    let t48172 = t48171 * t2392;
    let t48175 = t6710 * t6711 * t47953;
    let t48178 = t6583 * t12092 * t2478;
    let t48182 = F::cast_from(0.10725146985555128001e1_f64) * t557 * t1457 * t46915;
    let t48185 = F::cast_from(0.71500979903700853338e0_f64) * t1572 * t1457 * t46920;
    (t48172, t48175, t48178, t48182, t48185)
}
