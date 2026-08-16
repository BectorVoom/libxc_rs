//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1280/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1280(t3941: f64, t7769: f64, t1401: f64, t7467: f64, t1409: f64, t1419: f64, t56: f64, t6503: f64, t7251: f64, t67: f64, t1864: f64, t2109: f64, t7445: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7771 = 27.0_f64 * t3941 * t7769;
    let t7773 = 0.135e2_f64 * t1401 * t7467;
    let t7973 = -8.0_f64 / 3.0_f64 * t1419 * t56 - 5.0_f64 / 6.0_f64 * t7251 * t1409 + t6503;
    let t7974 = t7973 * t67;
    let t7975 = t7974 * t1864;
    let t7978 = t2109 * t7445;
    (t7771, t7773, t7973, t7974, t7975, t7978)
}
