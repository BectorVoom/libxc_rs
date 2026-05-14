//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1169/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1169<F: Float>(t10557: F, t6795: F, t8072: F, t9285: F, t20374: F, t7035: F, t993: F, t204: F, t34459: F, t7033: F, t20229: F, t6964: F, t10513: F, t20800: F, t4391: F, t10525: F, t1564: F, t539: F) -> (F, F, F, F, F, F, F) {
    let t34650 = 0.42900587942220512003e1 * t10557 * t6795;
    let t34652 = 0.71500979903700853338e0 * t9285 * t8072;
    let t34658 = t20374 * t993 * t7035;
    let t34659 = 0.38342925953920749676e0 * t34658;
    let t34662 = 0.92023022289409799224e1 * t7033 * t204 * t34459;
    let t34665 = 0.14300195980740170668e1 * t20229 * t6964 * t34459;
    let t34668 = 0.57200783922960682671e1 * t4391 * t20800 * t10513;
    let t34672 = 0.28600391961480341335e1 * t10525 * t539 * t1564 * t10513;
    (t34650, t34652, t34659, t34662, t34665, t34668, t34672)
}
