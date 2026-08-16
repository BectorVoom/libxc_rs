//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 726/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk726(t2440: f64, t327: f64, t10845: f64, t2347: f64, t2360: f64, t2923: f64, t13540: f64, t13542: f64, t4317: f64, t5: f64, t1882: f64, t4038: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14487 = t2440 * t327;
    let t14514 = t10845 * t2347;
    let t14519 = t2923 * t2360;
    let t14544 = 0.6419148148148148148e-1_f64 * t13540;
    let t14553 = 0.19257444444444444444e0_f64 * t13542;
    let t14571 = t5 * t4317;
    let t14635 = t1882 * t4038;
    (t14487, t14514, t14519, t14544, t14553, t14571, t14635)
}
