//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 482/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk482<F: Float>(t1645: F, t9182: F, t1457: F, t9177: F, t1450: F, t1456: F, t1572: F, t2385: F, t2386: F, t3159: F, t4446: F, t4507: F, t4540: F, t567: F, t574: F, t9310: F, t9313: F, t9318: F, t9321: F, t9324: F, t9327: F, t9330: F, t9333: F, t9339: F, t9342: F) -> (F, F) {
    let t9343 = t1645 * t9182;
    let t9346 = t1457 * t9177;
    let t9349 = -0.92023022289409799224e1 * t574 * t9310 - 0.11502877786176224903e2 * t1450 * t9313 + 0.46011511144704899612e1 * t567 * t9318 - 0.71500979903700853338e0 * t4507 * t9321 + 0.14300195980740170668e1 * t1572 * t9324 + 0.35750489951850426669e0 * t1456 * t9327 + 0.10725146985555128001e1 * t9330 * t4446 - 0.21450293971110256002e1 * t2385 * t9333 - 0.25025342966295298669e1 * t3159 * t2386 - 0.92023022289409799224e1 * t574 * t9339 + 0.42900587942220512003e1 * t9342 * t9343 - 0.21450293971110256001e1 * t4540 * t9346;
    (t9346, t9349)
}
