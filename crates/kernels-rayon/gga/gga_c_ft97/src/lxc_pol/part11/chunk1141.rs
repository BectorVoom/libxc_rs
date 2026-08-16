//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1141/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1141(t43396: f64, t43450: f64, t43508: f64, t43942: f64, t10660: f64, t870: f64, t875: f64, t2859: f64, t8232: f64, t2850: f64, t2854: f64, t10728: f64, t1882: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43944 = t43396 + t43450 + t43508 + t43942;
    let t43947 = t10660 * t870;
    let t43948 = t43947 * t875;
    let t43977 = t8232 * t2859;
    let t43979 = t8232 * t2850;
    let t43981 = t8232 * t2854;
    let t43983 = t1882 * t10728;
    (t43944, t43948, t43977, t43979, t43981, t43983)
}
