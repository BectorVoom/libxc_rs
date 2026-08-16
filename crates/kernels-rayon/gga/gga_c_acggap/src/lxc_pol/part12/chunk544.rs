//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 544/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk544(t378: f64, t968: f64, t177: f64, t377: f64, t973: f64, t1963: f64, t22: f64, t161: f64, t151: f64, t415: f64, t1077: f64, t145: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3552 = t378 * t968;
    let t3556 = 0.17006693853500995666e-1_f64 * t377 * t973 * t177;
    let t3558 = 1.0_f64 / t22 / t1963;
    let t3559 = t161 * t3558;
    let t3562 = 0.37792653007779990369e-1_f64 * t151 * t3559 * t177;
    let t3563 = t415 * t968;
    let t3565 = t1077 * t145;
    (t3552, t3556, t3558, t3562, t3563, t3565)
}
