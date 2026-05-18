//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 387/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk387<F: Float>(t2: F, t2360: F, t2371: F, t2334: F, t681: F, t756: F, t89: F, t2399: F, t259: F, t1882: F, t731: F, t768: F) -> (F, F, F, F, F, F, F, F) {
    let t2497 = t2 * t2360;
    let t2506 = t2371 * t2;
    let t2518 = F::new(4.0) / F::new(9.0) * t2334;
    let t2533 = F::new(4.0) / F::new(27.0) * t2334;
    let t2549 = t89 * t681 * t756;
    let t2553 = F::new(4.0) / F::new(27.0) * t89 * t2399 * t259;
    let t2554 = t1882 * t731;
    let t2556 = t1882 * t768;
    (t2497, t2506, t2518, t2533, t2549, t2553, t2554, t2556)
}
