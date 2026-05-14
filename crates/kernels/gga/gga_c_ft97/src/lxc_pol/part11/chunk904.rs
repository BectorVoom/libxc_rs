//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 904/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk904<F: Float>(t1526: F, t1944: F, t38308: F, t1970: F, t7705: F, t8779: F, t11262: F, t8767: F, t342: F, t630: F, t8783: F, t142: F, t7800: F, t8775: F, t15567: F, t16633: F, t3088: F, t7765: F, t7807: F, t8788: F, t8790: F, t9050: F) -> (F,) {
    let t41332 = t1526 * t38308 * t1944;
    let t41335 = t1526 * t7705 * t1970;
    let t41338 = t1526 * t7705 * t8779;
    let t41341 = t1526 * t11262 * t8767;
    let t41344 = t342 * t630 * t8783;
    let t41349 = t142 * t7800;
    let t41358 = t1526 * t7705 * t8775;
    let t41360 = 2.0 * t8790 + t41332 / 18.0 - t41335 / 6.0 - t41338 / 12.0 - t41341 / 9.0 + t8788 - t41344 / 4.0 - t1526 * t3088 * t9050 / 3.0 + 2.0 / 3.0 * t1526 * t3088 * t41349 * t7765 - t15567 * t16633 * t7807 / 3.0 + t41358 / 6.0;
    (t41360,)
}
