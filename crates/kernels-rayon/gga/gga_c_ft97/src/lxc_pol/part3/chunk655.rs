//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 655/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk655(t1526: f64, t1944: f64, t7705: f64, t1948: f64, t342: f64, t630: f64, t520: f64, t7773: f64, t89: f64, t548: f64, t8078: f64, t40: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8761 = t1526 * t7705 * t1944;
    let t8764 = t342 * t630 * t1948;
    let t8796 = t89 * t7773 * t520;
    let t8906 = t548 * t548;
    let t8907 = 1.0_f64 / t8906;
    let t8914 = 0.18521666970164609055e-1_f64 * t8078;
    let t8946 = t6 / t40;
    (t8761, t8764, t8796, t8907, t8914, t8946)
}
