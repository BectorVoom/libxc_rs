//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 404/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk404<F: Float>(t1037: F, t458: F, t1017: F, t2: F, t3318: F, t3335: F, t1030: F, t1882: F, t1055: F, t1045: F, t604: F) -> (F, F, F, F, F, F, F) {
    let t3513 = t458 * t1037;
    let t3518 = t2 * t1017;
    let t3530 = t3318 / F::cast_from(27.0_f64);
    let t3535 = t3335 / F::cast_from(9.0_f64);
    let t3545 = t1882 * t1030;
    let t3551 = t1882 * t1055;
    let t3578 = t1045 * t604;
    (t3513, t3518, t3530, t3535, t3545, t3551, t3578)
}
