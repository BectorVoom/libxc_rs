//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1120/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1120(t23077: f64, t6604: f64, t23083: f64, t7500: f64, t1509: f64, t236: f64, t23110: f64, t232: f64, t23109: f64, t1496: f64, t23069: f64, t1512: f64, t23041: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25119 = t23077 * t6604;
    let t25126 = t23083 * t7500;
    let t25130 = t236 * t1509;
    let t25132 = t23110 * t25130 * t232;
    let t25133 = t23109 * t25132;
    let t25140 = t23069 * t1496;
    let t25144 = t23041 * t1512;
    (t25119, t25126, t25132, t25133, t25140, t25144)
}
