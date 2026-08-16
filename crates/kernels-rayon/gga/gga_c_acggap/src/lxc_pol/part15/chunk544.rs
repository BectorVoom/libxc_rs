//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 544/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk544(t3937: f64, t865: f64, t191: f64, t813: f64, t4: f64, t483: f64, t657: f64, t1357: f64, t807: f64, t2847: f64, t1388: f64, t224: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3939 = 0.39512695097613069591e1_f64 * t3937 * t865;
    let t3952 = 1.0_f64 / t813 / t191;
    let t3992 = t483 * t4;
    let t3993 = t3992 * t657;
    let t4030 = t1357 * t807;
    let t4044 = 32.0_f64 * t2847;
    let t4045 = t224 * t1388;
    (t3939, t3952, t3993, t4030, t4044, t4045)
}
