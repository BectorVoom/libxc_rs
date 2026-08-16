//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 693/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk693(t100: f64, t1587: f64, t487: f64, t942: f64, t1882: f64, t3231: f64, t3201: f64, t8392: f64, t3170: f64, t8232: f64, t955: f64, t3227: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11810 = t1587 * t100;
    let t11811 = t487 * t942;
    let t11821 = 2.0_f64 / 9.0_f64 * t1882 * t3231;
    let t11826 = 2.0_f64 / 27.0_f64 * t8392 * t3201;
    let t11837 = t3170 * t487;
    let t11846 = t8232 * t955;
    let t11849 = 2.0_f64 / 9.0_f64 * t1882 * t3227;
    (t11810, t11811, t11821, t11826, t11837, t11846, t11849)
}
