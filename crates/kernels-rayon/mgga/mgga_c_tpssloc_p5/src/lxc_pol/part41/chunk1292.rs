//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1292/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1292(t103: f64, t1453: f64, t112: f64, t30349: f64, t111: f64, t8283: f64, t580: f64, t1404: f64, t1858: f64, t8199: f64, t2205: f64, t5381: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t111134 = t103 * t1453;
    let t111226 = t30349 * t112;
    let t111246 = t8283 * t111;
    let t111289 = 2.0_f64 * t30349 * t580;
    let t111291 = 2.0_f64 * t8283 * t1404;
    let t111293 = 2.0_f64 * t8199 * t1858;
    let t111302 = 2.0_f64 * t2205 * t5381;
    (t111134, t111226, t111246, t111289, t111291, t111293, t111302)
}
