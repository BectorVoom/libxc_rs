//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 896/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk896(t299: f64, t36047: f64, t36275: f64, t332: f64, t113: f64, t1275: f64, t34341: f64, t5: f64, t7692: f64, t992: f64, t2: f64, t7242: f64, t14: f64, t7469: f64) -> (f64, f64, f64, f64, f64) {
    let t300 = 10000000.0_f64 <= t299;
    let t36276 = t36047 + t36275;
    let t36277 = t36276 * t332;
    let t36285 = piecewise3(t300, 0.0_f64, t5 * t36277 * t113 / 4.0_f64 + t5 * t7692 * t992 / 4.0_f64 + t34341 * t1275 / 4.0_f64);
    let t36452 = t7242 * t2;
    let t36791 = t7469 * t14;
    (t36276, t36277, t36285, t36452, t36791)
}
