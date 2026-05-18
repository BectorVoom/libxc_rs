//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 894/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk894<F: Float>(t342: F, t630: F, t7729: F, t344: F, t8639: F, t7800: F, t81: F, t1526: F, t7705: F, t7721: F, t1533: F, t2252: F) -> (F, F, F, F, F) {
    let t38341 = t342 * t630 * t7729;
    let t38355 = F::new(5.0) / F::new(54.0) * t342 * t8639 * t344;
    let t38357 = t81 * t7800;
    let t38366 = t1526 * t7705 * t7721;
    let t38369 = t342 * t2252 * t1533;
    (t38341, t38355, t38357, t38366, t38369)
}
