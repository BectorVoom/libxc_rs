//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1629/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1629(t3131: f64, t6739: f64, t3215: f64, t390: f64, t300: f64, t3368: f64, t268: f64, t405: f64, t6546: f64) -> (f64, f64, f64, f64) {
    let t11066 = t6739 * t3131;
    let t11094 = 1.0_f64 / t3215 / t390;
    let t11126 = t300 * t3368;
    let t11135 = t268 * t6546 * t405;
    (t11066, t11094, t11126, t11135)
}
