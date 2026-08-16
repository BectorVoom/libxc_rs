//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1014/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1014<F: Float>(t1526: F, t1944: F, t38308: F, t1970: F, t7705: F, t8779: F, t11262: F, t8767: F, t342: F, t630: F, t8783: F, t142: F, t7800: F) -> (F, F, F, F, F, F) {
    let t41332 = t1526 * t38308 * t1944;
    let t41335 = t1526 * t7705 * t1970;
    let t41338 = t1526 * t7705 * t8779;
    let t41341 = t1526 * t11262 * t8767;
    let t41344 = t342 * t630 * t8783;
    let t41349 = t142 * t7800;
    (t41332, t41335, t41338, t41341, t41344, t41349)
}
