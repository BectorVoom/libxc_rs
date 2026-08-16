//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 236/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk236<F: Float>(t2334: F, t681: F, t756: F, t89: F, t2399: F, t259: F, t1882: F, t731: F, t768: F, t257: F, t760: F) -> (F, F, F, F, F, F, F) {
    let t2518 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2334;
    let t2533 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t2334;
    let t2549 = t89 * t681 * t756;
    let t2553 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t89 * t2399 * t259;
    let t2554 = t1882 * t731;
    let t2556 = t1882 * t768;
    let t2567 = F::cast_from(1.0_f64) / t760 / t257;
    (t2518, t2533, t2549, t2553, t2554, t2556, t2567)
}
