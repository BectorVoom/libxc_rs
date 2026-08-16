//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1106/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1106(t1410: f64, t407: f64, t1931: f64, t980: f64, t377: f64, t6552: f64, t1160: f64, t4210: f64, t6465: f64, t4180: f64, t6483: f64, t3088: f64, t4183: f64, t6482: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19834 = t407 * t1410;
    let t19838 = t980 * t1931;
    let t19840 = t377 * t6552;
    let t19843 = t1160 * t6465 * t4210;
    let t19845 = t4180 * t6483;
    let t19854 = t3088 * t6482 * t4183;
    (t19834, t19838, t19840, t19843, t19845, t19854)
}
