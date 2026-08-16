//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1077/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1077(t4180: f64, t4203: f64, t377: f64, t5307: f64, t1160: f64, t1629: f64, t16548: f64, t13584: f64, t16171: f64, t1004: f64, t5304: f64, t3088: f64, t4146: f64, t4183: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19208 = t4180 * t4203;
    let t19213 = t377 * t5307;
    let t19216 = t1160 * t1629 * t16548;
    let t19222 = t13584 * t1629 * t16171;
    let t19224 = t1004 * t5304;
    let t19235 = t3088 * t4146 * t4183;
    (t19208, t19213, t19216, t19222, t19224, t19235)
}
