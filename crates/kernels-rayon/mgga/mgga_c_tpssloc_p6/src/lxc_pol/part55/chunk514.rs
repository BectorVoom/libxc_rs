//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 514/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk514(t3545: f64, t456: f64, t1197: f64, t135: f64, t1174: f64, t121: f64, t486: f64, t1216: f64, t248: f64, t1213: f64, t478: f64, t483: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3547 = t456 * t3545 / 432.0_f64;
    let t3548 = t135 * t1197;
    let t3549 = t1174 * t3548;
    let t3570 = t121 * t486;
    let t3572 = t248 * t3570 * t1216;
    let t3573 = t1213 * t3572;
    let t3575 = t478 * t483;
    (t3547, t3548, t3549, t3570, t3572, t3573, t3575)
}
