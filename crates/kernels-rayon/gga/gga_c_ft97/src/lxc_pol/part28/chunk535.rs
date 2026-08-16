//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 535/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk535(t14: f64, t5589: f64, t70: f64, t444: f64, t5596: f64, t3076: f64, t1609: f64, t8: f64, t5566: f64, t1608: f64, t1669: f64, t5597: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22514 = t5589 * t14;
    let t22515 = t22514 * t70;
    let t22521 = t5596 * t444;
    let t22522 = t3076 * t22521;
    let t22532 = t8 * t1609;
    let t22533 = t5566 * t22532;
    let t22534 = t1608 * t22533;
    let t22540 = t5566 * t444;
    let t22541 = t1669 * t22540;
    let t22552 = t1669 * t5597;
    (t22514, t22515, t22522, t22532, t22534, t22540, t22541, t22552)
}
