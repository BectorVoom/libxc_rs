//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 840/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk840<F: Float>(t14392: F, t1445: F, t2194: F, t3040: F, t3720: F, t43718: F, t43756: F, t43758: F, t43760: F, t45560: F, t45563: F, t45565: F, t45569: F, t45575: F, t45577: F, t45580: F, t45586: F, t45588: F, t45598: F, t45600: F, t45603: F, t47508: F, t813: F, t8528: F) -> (F,) {
    let t50179 = -t45560 - t45563 + 0.71500979903700853338e0 * t47508 * t3040 - t45565 + 0.31952438294933958063e0 * t43718 - t45569 + t45575 + t45577 - t45580 + t45586 - t45588 + 0.38342925953920749676e1 * t43756 - 0.92023022289409799224e1 * t2194 * t14392 - 0.92023022289409799224e1 * t813 * t1445 * t8528 * t3720 - 0.51123901271894332901e1 * t43758 + 0.38342925953920749676e1 * t43760 + t45598 + t45600 + t45603;
    (t50179,)
}
