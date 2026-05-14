//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1059/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1059<F: Float>(t11262: F, t1526: F, t8767: F, t342: F, t630: F, t8783: F, t7705: F, t8775: F, t11119: F, t37940: F, t1690: F, t2037: F, t1614: F, t373: F, t7913: F, t929: F) -> (F, F, F, F, F, F, F) {
    let t41341 = t1526 * t11262 * t8767;
    let t41344 = t342 * t630 * t8783;
    let t41358 = t1526 * t7705 * t8775;
    let t44965 = t11119 * t37940;
    let t44969 = t1690 * t2037;
    let t44991 = t1614 * t373;
    let t45019 = t7913 * t929;
    (t41341, t41344, t41358, t44965, t44969, t44991, t45019)
}
