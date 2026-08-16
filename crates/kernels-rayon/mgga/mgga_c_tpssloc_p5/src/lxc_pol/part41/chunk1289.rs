//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1289/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1289(t2281: f64, t2331: f64, t656: f64, t9398: f64, t99: f64, t2196: f64, t2585: f64, t8181: f64, t8185: f64, t111: f64, t8199: f64, t9576: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t110140 = t2281 * t2331;
    let t110143 = t2281 * t656;
    let t110314 = t99 * t9398;
    let t110333 = 154.0_f64 / 27.0_f64 * t2585 * t2196;
    let t110334 = t110140 * t8181;
    let t110336 = t110143 * t8185;
    let t110363 = t8199 * t111;
    let t110532 = t9576 * t656;
    (t110140, t110143, t110314, t110333, t110334, t110336, t110363, t110532)
}
