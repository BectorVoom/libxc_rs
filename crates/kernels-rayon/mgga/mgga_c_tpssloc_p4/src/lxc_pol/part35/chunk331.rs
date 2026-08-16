//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 331/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk331(t1444: f64, t95: f64, t50: f64, t103: f64, t100: f64, t104: f64, t92: f64, tau1: f64) -> (f64, f64, f64, f64) {
    let t1445 = t95 * t1444;
    let t1447 = tau1 * t50;
    let t1449 = -t1444;
    let t1450 = t103 * t1449;
    let t1453 = 5.0_f64 / 3.0_f64 * t100 * t1450 - 5.0_f64 / 3.0_f64 * t1447 * t104 + 5.0_f64 / 3.0_f64 * t92 * t1445;
    (t1447, t1449, t1450, t1453)
}
