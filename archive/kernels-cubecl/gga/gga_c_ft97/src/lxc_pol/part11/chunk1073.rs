//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1073/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1073<F: Float>(t42009: F, t42025: F, t42042: F, t42250: F, t626: F, t703: F, t1526: F, t2322: F, t2355: F, t9483: F, t9503: F, t13598: F, t9491: F) -> (F, F, F, F, F, F) {
    let t42252 = t42009 + t42025 + t42042 + t42250;
    let t42262 = t626 * t703;
    let t42264 = t1526 * t42262 * t2322;
    let t42267 = t1526 * t9483 * t2355;
    let t42270 = t1526 * t9483 * t9503;
    let t42273 = t1526 * t13598 * t9491;
    (t42252, t42262, t42264, t42267, t42270, t42273)
}
