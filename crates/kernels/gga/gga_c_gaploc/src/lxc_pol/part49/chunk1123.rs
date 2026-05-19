//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1123/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1123<F: Float>(t13870: F, t795: F, t313: F, t2639: F, t13857: F, t4614: F, t813: F, t43682: F, t47296: F, t47299: F, t47303: F, t47306: F, t47309: F, t47315: F, t47317: F, t47321: F, t47325: F) -> (F, F) {
    let t47326 = t795 * t13870;
    let t47327 = t313 * t47326;
    let t47329 = F::cast_from(0.10725146985555128001e1_f64) * t47327 * t2639;
    let t47331 = t813 * t4614 * t13857;
    let t47333 = t43682 - F::cast_from(0.71500979903700853338e0_f64) * t47296 + F::cast_from(0.46011511144704899612e1_f64) * t47299 - t47303 + t47306 + t47309 + t47315 + t47317 + t47321 + t47325 - t47329 - F::cast_from(0.61348681526273199483e1_f64) * t47331;
    (t47326, t47333)
}
