//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 726/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk726<F: Float>(t2440: F, t327: F, t10845: F, t2347: F, t2360: F, t2923: F, t13540: F, t13542: F, t4317: F, t5: F, t1882: F, t4038: F) -> (F, F, F, F, F, F, F) {
    let t14487 = t2440 * t327;
    let t14514 = t10845 * t2347;
    let t14519 = t2923 * t2360;
    let t14544 = F::new(0.6419148148148148148e-1) * t13540;
    let t14553 = F::new(0.19257444444444444444e0) * t13542;
    let t14571 = t5 * t4317;
    let t14635 = t1882 * t4038;
    (t14487, t14514, t14519, t14544, t14553, t14571, t14635)
}
