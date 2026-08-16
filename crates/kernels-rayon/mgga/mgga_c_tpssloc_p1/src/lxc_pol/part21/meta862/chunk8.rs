//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3137/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3137(t43855: f64, t43859: f64, t43861: f64, t43863: f64, t44466: f64, t50968: f64, t50970: f64, t50972: f64, t50978: f64, t64003: f64, t64006: f64, t64045: f64) -> f64 {
    let t64929 = 4.0_f64 / 3.0_f64 * t64003 - 4.0_f64 * t64006 - t44466 + 5.0_f64 / 81.0_f64 * t43855 + 80.0_f64 / 81.0_f64 * t43859 - 5.0_f64 / 27.0_f64 * t43861 - 10.0_f64 / 27.0_f64 * t43863 - 4.0_f64 / 27.0_f64 * t50968 - 2.0_f64 / 27.0_f64 * t50970 - 4.0_f64 / 9.0_f64 * t50972 + 8.0_f64 / 81.0_f64 * t50978 - t64045 / 6.0_f64;
    t64929
}
