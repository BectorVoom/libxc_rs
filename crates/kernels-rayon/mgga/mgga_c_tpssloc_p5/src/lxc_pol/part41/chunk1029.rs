//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1029/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1029(t1336: f64, t16397: f64, t5252: f64, t225: f64, t5319: f64, t5217: f64, t1390: f64, t5356: f64, t112: f64, t5363: f64, t111: f64, t1851: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16398 = t1336 * t16397;
    let t16400 = 7.0_f64 / 1152.0_f64 * t16398 * t5252;
    let t16439 = t5319 * t225;
    let t16460 = t5217 * t225;
    let t16497 = t5356 * t1390;
    let t16521 = t5363 * t112;
    let t16524 = t1851 * t111;
    (t16400, t16439, t16460, t16497, t16521, t16524)
}
