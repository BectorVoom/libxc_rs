//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3138/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3138(t51039: f64, t51041: f64, t51043: f64, t51051: f64, t51053: f64, t64074: f64, t64076: f64, t64079: f64, t64082: f64, t64085: f64, t64087: f64, t64089: f64, t64092: f64) -> f64 {
    let t64943 = -40.0_f64 / 27.0_f64 * t51039 + 4.0_f64 / 9.0_f64 * t51041 + 4.0_f64 / 3.0_f64 * t51043 + 20.0_f64 / 81.0_f64 * t51051 + 8.0_f64 / 9.0_f64 * t51053 - 4.0_f64 / 27.0_f64 * t64074 - 4.0_f64 / 9.0_f64 * t64076 + t64079 / 9.0_f64 + t64082 / 3.0_f64 + 2.0_f64 * t64085 + 8.0_f64 / 9.0_f64 * t64087 + 4.0_f64 / 3.0_f64 * t64089 - 2.0_f64 / 3.0_f64 * t64092;
    t64943
}
