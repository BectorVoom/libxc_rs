//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1060/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1060<F: Float>(t52: F, t7883: F, t1630: F, t3056: F, t1712: F, t35: F, t533: F, t929: F, t7905: F, t383: F, t428: F, t37482: F, t11120: F, t1595: F, t37452: F, t1593: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t45082 = t52 * t7883;
    let t45110 = t1630 * t3056;
    let t45144 = t35 * t1712;
    let t45169 = t533 * t929;
    let t45461 = t7905 * t929;
    let t45488 = t428 * t383;
    let t45499 = t37482 * t383;
    let t45500 = t45499 * t11120;
    let t45526 = t37452 * t1595;
    let t45527 = t45526 * t11120;
    let t45540 = t1593 * t3056;
    (t45082, t45110, t45144, t45169, t45461, t45488, t45499, t45500, t45526, t45527, t45540)
}
