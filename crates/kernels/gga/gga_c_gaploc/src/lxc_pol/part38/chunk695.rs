//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 695/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk695<F: Float>(t13378: F, t11430: F, t901: F, t1000: F, t10497: F, t12446: F, t12450: F, t13354: F, t13356: F, t13360: F, t13365: F, t13370: F, t13374: F, t13375: F, t2859: F, t574: F) -> F {
    let t13379 = F::new(0.14896037479937677779e-1) * t13378;
    let t13380 = t11430 * t901;
    let t13381 = F::new(0.14896037479937677779e-1) * t13380;
    let t13382 = t13354 + t13356 + F::new(0.71500979903700853338e0) * t1000 * t10497 - F::new(0.92023022289409799224e1) * t574 * t13360 + t13365 - F::new(0.63904876589867916126e-1) * t12446 + F::new(0.63904876589867916126e-1) * t12450 - t13370 - t13374 - F::new(0.21450293971110256002e1) * t2859 * t13375 + t13379 + t13381;
    t13382
}
