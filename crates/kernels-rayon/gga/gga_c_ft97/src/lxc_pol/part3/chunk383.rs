//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 383/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk383(t2347: f64, t2440: f64, t2360: f64, t703: f64, t754: f64, t761: f64, t250: f64, t251: f64, t1771: f64, t249: f64, t1775: f64, t740: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2441 = t2440 * t2347;
    let t2446 = t703 * t2360;
    let t2469 = t754 * t761;
    let t2475 = 1.0_f64 / t251 / t250;
    let t2481 = 4.0_f64 / 9.0_f64 * t1771 * t249;
    let t2482 = t1775 * t740;
    (t2441, t2446, t2469, t2475, t2481, t2482)
}
