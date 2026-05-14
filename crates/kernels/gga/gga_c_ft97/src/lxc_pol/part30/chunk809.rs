//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 809/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk809<F: Float>(t10491: F, t1240: F, t1160: F, t2372: F, t222: F, t2382: F, t226: F, t1689: F, t3771: F, t6813: F, t3722: F, t2378: F, t37481: F, t223: F, t676: F, t2568: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t57089 = t10491 * t1240;
    let t65408 = t2372 * t1160;
    let t65692 = t2382 * t222;
    let t65693 = t65692 * t226;
    let t66076 = t3771 * t6813 * t1689;
    let t66382 = t3722 * t222;
    let t66422 = t37481 * t2378;
    let t66563 = t3722 * t223;
    let t67847 = t676 * t1160;
    let t67996 = t2372 * t2568;
    (t57089, t65408, t65692, t65693, t66076, t66382, t66422, t66563, t67847, t67996)
}
