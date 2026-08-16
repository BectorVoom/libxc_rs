//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 724/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk724(t1882: f64, t3856: f64, t3974: f64, t9735: f64, t9701: f64, t13746: f64, t13753: f64, t13780: f64, t13794: f64, t13809: f64, t13811: f64, t4354: f64, t8675: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14281 = 2.0_f64 / 27.0_f64 * t1882 * t3856;
    let t14283 = 2.0_f64 / 9.0_f64 * t1882 * t3974;
    let t14317 = 4.0_f64 / 81.0_f64 * t9735;
    let t14318 = 4.0_f64 / 27.0_f64 * t9701;
    let t14327 = 2.0_f64 / 9.0_f64 * t13746;
    let t14329 = t13753 / 9.0_f64;
    let t14336 = t13780 / 27.0_f64;
    let t14341 = 2.0_f64 / 81.0_f64 * t13794;
    let t14346 = t13809 / 27.0_f64;
    let t14347 = 2.0_f64 / 27.0_f64 * t13811;
    let t14421 = 4.0_f64 / 9.0_f64 * t8675 * t4354;
    (t14281, t14283, t14317, t14318, t14327, t14329, t14336, t14341, t14346, t14347, t14421)
}
