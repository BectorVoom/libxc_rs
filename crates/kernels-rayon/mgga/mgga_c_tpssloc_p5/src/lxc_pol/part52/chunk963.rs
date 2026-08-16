//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 963/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk963(t23482: f64, t6741: f64, t344: f64, t6729: f64, t6740: f64, t3103: f64, t6755: f64, t3034: f64, t371: f64, t1930: f64, t1940: f64, t3046: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23483 = t23482 * t6741;
    let t23488 = t6729 * t344;
    let t23489 = t6740 * t23488;
    let t23500 = t6755 * t3103;
    let t23508 = 1.0_f64 / t3034 / t371;
    let t23509 = t1930 * t23508;
    let t23528 = t1940 * t3046;
    (t23483, t23489, t23500, t23508, t23509, t23528)
}
