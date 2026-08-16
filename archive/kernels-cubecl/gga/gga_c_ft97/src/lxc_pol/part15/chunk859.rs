//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 859/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk859<F: Float>(t122: F, t31: F, t7911: F, t76: F, t8050: F, t66: F, t378: F, t7241: F, t1586: F, t1642: F, t422: F, t626: F) -> (F, F, F, F, F) {
    let t38211 = t122 / t31 / t7911;
    let t38241 = F::cast_from(1.0_f64) / t8050 / t76;
    let t38242 = t66 * t38241;
    let t38262 = t378 * t7241;
    let t38268 = t1642 * t1586;
    let t38308 = t626 * t422;
    (t38211, t38242, t38262, t38268, t38308)
}
