//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2549/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2549(t1020: f64, t10508: f64, t248: f64, t4650: f64, t10962: f64, t4630: f64, t13961: f64, t3114: f64, t10957: f64, t4571: f64, t13950: f64, t3048: f64) -> (f64, f64, f64, f64, f64) {
    let t49818 = t1020 * t248 * t10508 * t4650;
    let t49820 = t10962 * t4630;
    let t49822 = t3114 * t13961;
    let t49827 = t10957 * t4571;
    let t49829 = t3048 * t13950;
    (t49818, t49820, t49822, t49827, t49829)
}
