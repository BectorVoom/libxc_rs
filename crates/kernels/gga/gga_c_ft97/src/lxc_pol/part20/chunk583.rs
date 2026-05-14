//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 583/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk583<F: Float>(t3627: F, t41: F, t70: F, t171: F, t7741: F, t11: F, t3746: F, t713: F, t2493: F, t1934: F, t3699: F) -> (F, F, F, F, F) {
    let t12143 = t41 * t3627 * t70;
    let t12168 = 1.0 / t171 / t7741;
    let t12169 = t11 * t12168;
    let t12170 = t41 * t12169;
    let t13292 = t3746 * t713;
    let t13293 = t2493 * t13292;
    let t13296 = t3699 * t1934;
    (t12143, t12170, t13292, t13293, t13296)
}
