//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 527/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk527(t103: f64, t1449: f64, t100: f64, t104: f64, t1445: f64, t1447: f64, t92: f64) -> (f64, f64) {
    let t1450 = t103 * t1449;
    let t1453 = 5.0_f64 / 3.0_f64 * t100 * t1450 - 5.0_f64 / 3.0_f64 * t1447 * t104 + 5.0_f64 / 3.0_f64 * t92 * t1445;
    (t1450, t1453)
}
