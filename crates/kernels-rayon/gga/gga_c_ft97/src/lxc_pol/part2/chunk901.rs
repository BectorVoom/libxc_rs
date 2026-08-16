//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 901/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk901(t10085: f64, t3876: f64, t3881: f64, t9787: f64, t3877: f64, t8392: f64, t3882: f64, t3888: f64, t1882: f64, t3979: f64, t4005: f64, t713: f64, t729: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13952 = t10085 * t3876;
    let t13955 = t9787 * t3881;
    let t13959 = 2.0_f64 / 27.0_f64 * t8392 * t3877;
    let t13961 = 2.0_f64 / 27.0_f64 * t8392 * t3882;
    let t13963 = 4.0_f64 / 27.0_f64 * t8392 * t3888;
    let t13965 = 2.0_f64 / 9.0_f64 * t1882 * t3979;
    let t13967 = t729 * t4005 * t713;
    (t13952, t13955, t13959, t13961, t13963, t13965, t13967)
}
