//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 909/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk909(t1160: f64, t676: f64, t2372: f64, t2568: f64, t1240: f64, t2681: f64, t1200: f64, t7606: f64, t19106: f64, t800: f64, t4092: f64, t2843: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t67847 = t676 * t1160;
    let t67996 = t2372 * t2568;
    let t69996 = t2681 * t1240;
    let t70497 = t1200 * t7606;
    let t70550 = t800 * t19106;
    let t70779 = t4092 * t19106;
    let t72190 = t2681 * t2843;
    (t67847, t67996, t69996, t70497, t70550, t70779, t72190)
}
