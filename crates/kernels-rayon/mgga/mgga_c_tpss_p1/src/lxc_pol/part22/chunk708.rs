//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 708/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk708(t3432: f64, t70: f64, t1290: f64, t602: f64, t1306: f64, t582: f64, t1289: f64, t2009: f64, t581: f64, t3431: f64, t48: f64, t2016: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3433 = t3432 * t70;
    let t3436 = t1290 * t602;
    let t3441 = t582 * t1306;
    let t3446 = t2009 * t1289;
    let t3447 = t3446 * t581;
    let t3450 = t48 * t3431;
    let t3455 = t2016 * t1289;
    (t3433, t3436, t3441, t3446, t3447, t3450, t3455)
}
